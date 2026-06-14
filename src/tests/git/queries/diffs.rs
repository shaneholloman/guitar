use super::*;
use crate::git::actions::rebasing::{RebaseOutcome, start_rebase};
use git2::{Repository, Signature, build::CheckoutBuilder};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_repo(name: &str) -> (PathBuf, Repository) {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("guitar-diff-{name}-{id}"));
    fs::create_dir_all(&path).unwrap();
    let repo = Repository::init(&path).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
    }
    (path, repo)
}

fn write(path: &Path, file: &str, content: &str) {
    fs::write(path.join(file), content).unwrap();
}

fn commit(repo: &Repository, file: &str, message: &str) -> Oid {
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

fn checkout_new_branch(repo: &Repository, name: &str) {
    let head = repo.head().unwrap().peel_to_commit().unwrap();
    repo.branch(name, &head, false).unwrap();
    repo.set_head(&format!("refs/heads/{name}")).unwrap();
    repo.checkout_head(Some(CheckoutBuilder::default().force())).unwrap();
}

fn checkout_branch(repo: &Repository, name: &str) {
    repo.set_head(&format!("refs/heads/{name}")).unwrap();
    repo.checkout_head(Some(CheckoutBuilder::default().force())).unwrap();
}

#[test]
fn workdir_diff_marks_conflicted_paths() {
    let (path, repo) = temp_repo("conflict");
    write(&path, "file.txt", "base\n");
    commit(&repo, "file.txt", "base");
    checkout_new_branch(&repo, "feature");
    write(&path, "file.txt", "feature\n");
    commit(&repo, "file.txt", "feature");
    checkout_branch(&repo, "master");
    write(&path, "file.txt", "main\n");
    let main = commit(&repo, "file.txt", "main");
    checkout_branch(&repo, "feature");

    assert_eq!(start_rebase(&repo, main).unwrap(), RebaseOutcome::Conflict);

    let changes = get_filenames_diff_at_workdir(&repo).unwrap();
    assert!(changes.has_conflicts);
    assert!(changes.is_staged);
    assert!(changes.is_unstaged);
    assert_eq!(changes.conflict_count, 1);
    assert_eq!(changes.conflicts, vec!["file.txt".to_string()]);

    let conflict = get_conflict_file(&repo, "file.txt").unwrap().unwrap();
    assert!(!conflict.ours.is_empty());
    assert!(!conflict.theirs.is_empty());
    assert!(conflict.workdir.iter().any(|line| line.starts_with("<<<<<<<")));
    assert!(conflict.workdir.iter().any(|line| line.starts_with("=======")));
    assert!(conflict.workdir.iter().any(|line| line.starts_with(">>>>>>>")));

    let _ = fs::remove_dir_all(path);
}
