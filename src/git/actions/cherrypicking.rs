use git2::CherrypickOptions;
use git2::{Error, Oid, Repository, build::CheckoutBuilder};

pub fn cherry_pick_commit(
    repo: &Repository,
    commit_oid: Oid,
    message: Option<&str>, // Optional override for the new commit message.
    allow_conflicts: bool, // When true, accept our side of conflicted index entries.
) -> Result<Oid, Error> {
    let commit = repo.find_commit(commit_oid)?;

    // The current HEAD becomes the parent of the synthesized cherry-pick commit.
    let head_commit = repo.head()?.peel_to_commit()?;

    let mut cherrypick_opts = CherrypickOptions::new();

    // Libgit2 applies the patch into the index; this function commits it afterward.
    repo.cherrypick(&commit, Some(&mut cherrypick_opts))?;

    let mut index = repo.index()?;

    if index.has_conflicts() {
        if allow_conflicts {
            // Keep the existing side for conflicts so the command can finish without a merge UI.
            let conflicts: Vec<_> = index.conflicts()?.flatten().filter_map(|e| e.our).collect();
            for conflict in conflicts {
                index.add(&conflict)?;
            }
        } else {
            return Err(Error::from_str("Cherry-pick conflicts detected"));
        }
    }

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let sig = repo.signature()?;

    // Reuse the source message unless a caller supplied a contextual one.
    let commit_message = message.unwrap_or_else(|| commit.message().unwrap_or("Cherry-pick commit"));

    let parents = [&head_commit];

    let new_commit_oid = repo.commit(Some("HEAD"), &sig, &sig, commit_message, &tree, &parents)?;

    // Make the working tree match the committed index after the cherry-pick write.
    repo.checkout_head(Some(CheckoutBuilder::default().allow_conflicts(allow_conflicts).force()))?;

    Ok(new_commit_oid)
}
