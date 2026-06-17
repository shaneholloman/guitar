use crate::git::queries::{commits::get_stashed_commits, reflogs::HeadReflogEntry};
use crate::{
    core::{
        batcher::Batcher,
        buffer::Buffer,
        chunk::{Chunk, NONE},
        oids::Oids,
    },
    git::queries::commits::{get_sorted_oids, get_tag_oids, get_tip_oids},
    git::queries::reflogs::get_head_reflog_entries,
};
use git2::Repository;
use im::HashSet;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Walks git history into lane snapshots and ref lookup tables.
pub struct Walker {
    // Repository state shared with the batcher and stash query.
    pub repo: Rc<RefCell<Repository>>,

    // Revwalk cursor for incremental history loading.
    pub batcher: Batcher,

    // Mutable lane buffer that records topology deltas.
    pub buffer: RefCell<Buffer>,

    // Alias and ref metadata accumulated during the walk.
    pub oids: Oids,

    pub branches_lanes: HashMap<u32, usize>,
    pub branches_local: HashMap<u32, Vec<String>>,
    pub branches_remote: HashMap<u32, Vec<String>>,

    pub tags_lanes: HashMap<u32, usize>,
    pub tags_local: HashMap<u32, Vec<String>>,

    pub stashes_lanes: HashMap<u32, usize>,
    pub reflogs_lanes: HashMap<u32, usize>,
    pub head_reflog_entries: Vec<HeadReflogEntry>,

    // Number of commits requested per walk iteration.
    pub amount: usize,
}

impl Walker {
    // Open the repository and seed all metadata that does not depend on walking commits.
    pub fn new(path: String, amount: usize, hidden_branch_names: HashSet<String>, include_head_reflog_roots: bool) -> Result<Self, git2::Error> {
        let path = path.clone();
        let repo = Rc::new(RefCell::new(Repository::open(path).expect("Failed to open repo")));

        let buffer = RefCell::new(Buffer::default());

        let mut oids = Oids::default();

        // Branch and tag tips are registered before walking so aliases are stable.
        let branches_lanes = HashMap::new();
        let (branches_local, branches_remote) = get_tip_oids(&repo.borrow(), &mut oids);

        let tags_lanes = HashMap::new();
        let tags_local = get_tag_oids(&repo.borrow(), &mut oids);

        let stashes_lanes = HashMap::new();
        let reflogs_lanes = HashMap::new();

        // Stashes are collected up front so they can be inserted near their parents later.
        {
            let mut repo_mut = repo.borrow_mut();
            oids.stashes = get_stashed_commits(&mut repo_mut, &mut oids);
        }

        let head_reflog_entries = get_head_reflog_entries(&repo.borrow()).unwrap_or_default();
        let mut head_reflog_roots = Vec::new();
        for entry in &head_reflog_entries {
            oids.get_alias_by_oid(entry.new_oid);
            if include_head_reflog_roots && !head_reflog_roots.contains(&entry.new_oid) {
                head_reflog_roots.push(entry.new_oid);
            }
        }

        let batcher = Batcher::new(repo.clone(), &hidden_branch_names, &head_reflog_roots).expect("Error");

        Ok(Self { repo, batcher, buffer, oids, branches_lanes, branches_local, branches_remote, tags_lanes, tags_local, stashes_lanes, reflogs_lanes, head_reflog_entries, amount })
    }

    // Process one revwalk page and update lane snapshots for the renderer.
    pub fn walk(&mut self) -> bool {
        let repo = self.repo.borrow();

        // Without HEAD there is no stable parent for the uncommitted pseudo-row.
        let head_oid = match repo.head().ok().and_then(|h| h.target()) {
            Some(oid) => oid,
            None => {
                return false;
            },
        };

        let head_alias = self.oids.get_alias_by_oid(head_oid);

        let mut sorted_batch: Vec<u32> = Vec::new();
        get_sorted_oids(&self.batcher, &mut self.oids, &mut sorted_batch, self.amount);

        // Alias NONE is rendered as the uncommitted row above HEAD.
        if self.oids.get_commit_count() == 1 {
            self.buffer.borrow_mut().update(Chunk::uncommitted(head_alias, NONE));
        }

        let stashes: Vec<u32> = self.oids.stashes.clone();
        let reflog_aliases: Vec<u32> = self.head_reflog_entries.iter().filter_map(|entry| self.oids.aliases.get(&entry.new_oid).copied()).collect();

        // Place each stash near its first parent so it reads as a side snapshot.
        for &stash_alias in &stashes {
            let stash_oid = self.oids.get_oid_by_alias(stash_alias);
            let stash_commit = repo.find_commit(*stash_oid).unwrap();

            if let Some(parent_oid) = stash_commit.parent_ids().next() {
                let parent_alias = self.oids.get_alias_by_oid(parent_oid);

                if let Some(pos) = sorted_batch.iter().position(|&a| a == parent_alias) {
                    sorted_batch.insert(if pos == 0 { 0 } else { pos - 1 }, stash_alias);
                }
            }
        }

        // Hold one mutable buffer borrow while the page updates topology.
        let mut buffer = self.buffer.borrow_mut();

        for &alias in sorted_batch.iter() {
            let mut merger_alias: u32 = NONE;
            let oid = self.oids.get_oid_by_alias(alias);
            let commit = repo.find_commit(*oid).unwrap();

            // Only two parents are modeled because the renderer draws one merge edge.
            let mut parents_iter = commit.parent_ids();
            let parent_a_oid = parents_iter.next();
            let parent_b_oid = parents_iter.next();

            // Stashes should point only to their base commit, not the index/worktree parents.
            let (parent_a, parent_b) = if stashes.contains(&alias) {
                (parent_a_oid.map(|p| self.oids.get_alias_by_oid(p)).unwrap_or(NONE), NONE)
            } else {
                (parent_a_oid.map(|p| self.oids.get_alias_by_oid(p)).unwrap_or(NONE), parent_b_oid.map(|p| self.oids.get_alias_by_oid(p)).unwrap_or(NONE))
            };

            let chunk = Chunk::commit(alias, parent_a, parent_b);

            buffer.update(chunk);

            for (lane_idx, chunk) in buffer.curr.iter().enumerate() {
                if !chunk.is_dummy() && alias == chunk.alias {
                    // Ref lanes are captured after the buffer decides where this alias sits.
                    if self.branches_local.contains_key(&alias) || self.branches_remote.contains_key(&alias) {
                        self.branches_lanes.insert(alias, lane_idx);
                    }

                    if self.tags_local.contains_key(&alias) {
                        self.tags_lanes.insert(alias, lane_idx);
                    }

                    if stashes.contains(&alias) {
                        self.stashes_lanes.insert(alias, lane_idx);
                    }

                    if reflog_aliases.contains(&alias) {
                        self.reflogs_lanes.insert(alias, lane_idx);
                    }

                    if chunk.parent_a != NONE && chunk.parent_b != NONE {
                        // If the second parent is not already visible as a lane, mark a deferred merge.
                        let mut is_merger_found = false;
                        for chunk_nested in buffer.curr.iter() {
                            if chunk_nested.parent_a != NONE && chunk_nested.parent_b == NONE && chunk.parent_b == chunk_nested.parent_a {
                                is_merger_found = true;
                                break;
                            }
                        }
                        if !is_merger_found {
                            merger_alias = chunk.alias;
                        }
                    }
                }
            }

            if merger_alias != NONE {
                buffer.merger(merger_alias);
            }

            // Preserve the rendered order separately from first-seen alias assignment.
            self.oids.append_sorted_alias(alias);
        }

        // Empty pages mean the worker is done; emit one backup so lane-window reconstruction has a final delta.
        if sorted_batch.is_empty() {
            buffer.backup();
            return false;
        }

        true
    }
}

#[cfg(test)]
#[path = "../tests/core/walker.rs"]
mod tests;
