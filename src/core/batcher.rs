use git2::{BranchType, Oid, Repository, Revwalk};
use im::HashSet;
use std::cell::RefCell;
use std::{rc::Rc, sync::Mutex};

// Own the revwalk cursor so commit history can be loaded in pages.
pub struct Batcher {
    revwalk: Mutex<Revwalk<'static>>,
}

impl Batcher {
    // Build the initial revwalk from all visible local and remote branch tips.
    pub fn new(repo: Rc<RefCell<Repository>>, visible_branch_names: &HashSet<String>) -> Result<Self, git2::Error> {
        let revwalk = Self::build(&repo.borrow(), visible_branch_names)?;
        Ok(Self { revwalk: Mutex::new(revwalk) })
    }

    // Recreate the cursor after branch filters, fetches, or repository state changes.
    pub fn reset(&self, repo: Rc<RefCell<Repository>>, visible_branch_names: &HashSet<String>) -> Result<(), git2::Error> {
        let revwalk = Self::build(&repo.borrow(), visible_branch_names)?;
        let mut guard = self.revwalk.lock().unwrap();
        *guard = revwalk;
        Ok(())
    }

    // Pull the next page, dropping commits libgit2 cannot resolve.
    pub fn next(&self, count: usize) -> Vec<Oid> {
        let mut revwalk = self.revwalk.lock().unwrap();
        revwalk.by_ref().take(count).filter_map(Result::ok).collect()
    }

    fn build(repo: &Repository, visible_branch_names: &HashSet<String>) -> Result<Revwalk<'static>, git2::Error> {
        // The repository outlives the revwalk in App state; this keeps libgit2's lifetime usable here.
        let repo_ref: &'static Repository = unsafe { std::mem::transmute::<&Repository, &'static Repository>(repo) };

        let mut revwalk = repo_ref.revwalk()?;

        for branch_type in [BranchType::Local, BranchType::Remote] {
            for branch_result in repo.branches(Some(branch_type))? {
                let (branch, _) = branch_result?;

                let Some(oid) = branch.get().target() else { continue };

                let name = branch.name()?.unwrap_or("").to_string();

                // An empty filter means every branch contributes a tip.
                if visible_branch_names.is_empty() || visible_branch_names.contains(&name) {
                    revwalk.push(oid)?;
                }
            }
        }

        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;
        Ok(revwalk)
    }
}
