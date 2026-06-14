use git2::{Oid, Repository, Time};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadReflogEntry {
    pub selector: String,
    pub old_oid: Oid,
    pub new_oid: Oid,
    pub message: String,
    pub time: Time,
}

pub fn get_head_reflog_entries(repo: &Repository) -> Result<Vec<HeadReflogEntry>, git2::Error> {
    let reflog = repo.reflog("HEAD")?;
    let mut entries = Vec::new();

    for (idx, entry) in reflog.iter().enumerate() {
        let new_oid = entry.id_new();
        if new_oid.is_zero() || repo.find_commit(new_oid).is_err() {
            continue;
        }

        let message = entry.message().map(str::to_string).or_else(|| entry.message_bytes().map(|bytes| String::from_utf8_lossy(bytes).to_string())).unwrap_or_else(|| "reflog".to_string());

        entries.push(HeadReflogEntry { selector: format!("HEAD@{{{idx}}}"), old_oid: entry.id_old(), new_oid, message, time: entry.committer().when() });
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, ResetType, Signature};
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_repo(name: &str) -> (PathBuf, Repository) {
        let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("guitar-reflog-query-{name}-{id}"));
        fs::create_dir_all(&path).unwrap();
        let repo = Repository::init(&path).unwrap();
        {
            let mut config = repo.config().unwrap();
            config.set_str("user.name", "Test User").unwrap();
            config.set_str("user.email", "test@example.com").unwrap();
        }
        (path, repo)
    }

    fn commit(repo: &Repository, file: &str, message: &str) -> Oid {
        let workdir = repo.workdir().unwrap().to_path_buf();
        fs::write(workdir.join(file), message).unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new(file)).unwrap();
        index.write().unwrap();
        let tree_oid = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_oid).unwrap();
        let sig = Signature::now("Test User", "test@example.com").unwrap();
        let parent = repo.head().ok().and_then(|head| head.peel_to_commit().ok());
        let parents: Vec<&git2::Commit<'_>> = parent.iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents).unwrap()
    }

    #[test]
    fn head_reflog_keeps_commit_after_reset() {
        let (_path, repo) = temp_repo("lost-head");
        let base = commit(&repo, "file.txt", "base");
        let lost = commit(&repo, "file.txt", "lost");
        let base_commit = repo.find_commit(base).unwrap();
        repo.reset(base_commit.as_object(), ResetType::Hard, None).unwrap();

        let entries = get_head_reflog_entries(&repo).unwrap();

        assert!(entries.iter().any(|entry| entry.new_oid == lost && entry.selector.starts_with("HEAD@{")));
        assert_eq!(repo.head().unwrap().target(), Some(base));
    }
}
