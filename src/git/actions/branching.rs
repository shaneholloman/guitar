use git2::{BranchType, Error, Oid, Repository};

use crate::git::actions::pushing::delete_remote_branch_ssh;

pub fn create_branch(repo: &Repository, branch_name: &str, target_oid: Oid) -> Result<(), Error> {
    // Branch creation is intentionally non-checkout; the graph stays on the current HEAD.
    let target_commit = repo.find_commit(target_oid)?;

    repo.branch(branch_name, &target_commit, false)?;

    Ok(())
}

pub fn delete_branch(repo: &Repository, branch: &str) -> Result<(), git2::Error> {
    // Prefer local deletion, because names without a slash are local in the branch pane.
    if let Ok(mut local) = repo.find_branch(branch, BranchType::Local) {
        local.delete()?;
        return Ok(());
    }

    // If no local branch exists, treat the name as remote/branch and delete on the remote.
    let (remote_name, remote_branch) = if let Some((remote, b)) = branch.split_once('/') { (remote, b) } else { ("origin", branch) };

    let repo_path = repo.path().to_str().ok_or_else(|| git2::Error::from_str("Invalid repo path"))?;
    let handle = delete_remote_branch_ssh(repo_path, remote_name, remote_branch);

    // Join here so callers see the same Result shape for local and remote deletion.
    handle.join().map_err(|_| git2::Error::from_str("Failed to join remote deletion thread"))??;

    Ok(())
}
