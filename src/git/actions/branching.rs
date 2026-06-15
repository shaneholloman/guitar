use git2::{BranchType, Error, Oid, Repository};

pub fn create_branch(repo: &Repository, branch_name: &str, target_oid: Oid) -> Result<(), Error> {
    // Branch creation is intentionally non-checkout; the graph stays on the current HEAD.
    let target_commit = repo.find_commit(target_oid)?;

    repo.branch(branch_name, &target_commit, false)?;

    Ok(())
}

pub fn delete_branch(repo: &Repository, branch: &str) -> Result<(), git2::Error> {
    let mut local = repo.find_branch(branch, BranchType::Local)?;
    local.delete()?;
    Ok(())
}
