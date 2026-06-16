use crate::git::queries::helpers::FileStatus;
use git2::{Delta, DiffFindOptions, DiffOptions, Oid, Repository};
use std::path::Path;

pub fn changed_file_status_at_commit(repo: &Repository, oid: Oid, path: &str) -> Result<Option<FileStatus>, git2::Error> {
    let path = normalize_path(path);
    if path.is_empty() {
        return Ok(None);
    }

    let commit = repo.find_commit(oid)?;
    let tree = commit.tree()?;
    let parent_tree = if commit.parent_count() > 0 { Some(commit.parent(0)?.tree()?) } else { None };

    let mut opts = DiffOptions::new();
    opts.include_untracked(false).recurse_untracked_dirs(false).include_typechange(false).ignore_submodules(true).show_binary(false).minimal(false).skip_binary_check(true);

    let mut diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut opts))?;
    let mut find_options = DiffFindOptions::new();
    find_options.renames(true);
    diff.find_similar(Some(&mut find_options))?;
    for delta in diff.deltas() {
        if !delta_matches_path(&delta, &path) {
            continue;
        }

        return Ok(Some(file_status(delta.status())));
    }

    Ok(None)
}

fn delta_matches_path(delta: &git2::DiffDelta<'_>, path: &str) -> bool {
    let selected = Path::new(path);
    delta.old_file().path().is_some_and(|old_path| old_path == selected) || delta.new_file().path().is_some_and(|new_path| new_path == selected)
}

fn file_status(delta: Delta) -> FileStatus {
    match delta {
        Delta::Added => FileStatus::Added,
        Delta::Modified => FileStatus::Modified,
        Delta::Deleted => FileStatus::Deleted,
        Delta::Renamed => FileStatus::Renamed,
        _ => FileStatus::Other,
    }
}

fn normalize_path(path: &str) -> String {
    let normalized = path.trim().replace('\\', "/");
    strip_leading_dot_slashes(&normalized).to_string()
}

fn strip_leading_dot_slashes(mut path: &str) -> &str {
    while let Some(stripped) = path.strip_prefix("./") {
        path = stripped;
    }
    path
}

#[cfg(test)]
#[path = "../../tests/git/queries/file_history.rs"]
mod tests;
