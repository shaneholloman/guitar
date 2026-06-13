use git2::{Error, Repository, ResetType, StatusOptions};

pub fn stage_all(repo: &Repository) -> Result<(), Error> {
    let mut index = repo.index()?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true).include_ignored(false).include_unmodified(false);

    let statuses = repo.statuses(Some(&mut opts))?;

    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            let path = std::path::Path::new(path);

            match entry.status() {
                s if s.is_wt_deleted() || s.is_index_deleted() => {
                    // A deleted tracked file is staged by removing its index entry.
                    if index.get_path(path, 0).is_some() {
                        index.remove_path(path)?;
                    }
                },
                _ => {
                    // New and modified files both enter the index through add_path.
                    index.add_path(path)?;
                },
            }
        }
    }

    index.write()?;
    Ok(())
}

pub fn unstage_all(repo: &Repository) -> Result<(), git2::Error> {
    let head = match repo.head() {
        Ok(head) => head.peel_to_commit()?,
        Err(_) => {
            // A fresh repository has no HEAD to reset back to.
            return Ok(());
        },
    };

    // Mixed reset keeps workdir changes while returning the whole index to HEAD.
    repo.reset(&head.into_object(), ResetType::Mixed, None)?;

    Ok(())
}

pub fn stage_file(repo: &Repository, path: &std::path::Path) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    if path.exists() {
        // Existing paths represent new or modified files.
        index.add_path(path)?;
    } else {
        // Missing paths represent deletes, which are staged by removing index entries.
        index.remove_path(path)?;
    }

    index.write()?;
    Ok(())
}

pub fn unstage_file(repo: &Repository, path: &std::path::Path) -> Result<(), git2::Error> {
    let head = match repo.head() {
        Ok(h) => h.peel_to_commit()?,
        Err(_) => {
            // Without HEAD, unstage means remove the path from the initial index.
            let mut index = repo.index()?;
            index.remove_path(path)?;
            index.write()?;
            return Ok(());
        },
    };

    // reset_default updates the index pathspec without touching working tree contents.
    repo.reset_default(Some(&head.into_object()), [path])?;
    Ok(())
}
