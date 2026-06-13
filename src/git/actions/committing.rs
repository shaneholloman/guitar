use git2::{Error, ErrorCode, Oid, Repository, Signature};

pub fn commit_staged(repo: &Repository, message: &str, name: &str, email: &str) -> Result<Oid, Error> {
    let mut index = repo.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    // A normal commit uses HEAD as its parent; an unborn branch creates the root commit.
    let parent_commit = match repo.head() {
        Ok(head_ref) => head_ref.peel_to_commit().ok(),
        Err(e) => {
            if e.code() == ErrorCode::UnbornBranch {
                None
            } else {
                return Err(e);
            }
        },
    };

    let signature = Signature::now(name, email)?;

    let commit_oid = if let Some(parent) = parent_commit {
        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent])?
    } else {
        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?
    };

    Ok(commit_oid)
}
