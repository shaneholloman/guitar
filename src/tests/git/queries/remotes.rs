use super::*;
use git2::Repository;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_repo(name: &str) -> (std::path::PathBuf, Repository) {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("guitar-remote-query-{name}-{id}"));
    fs::create_dir_all(&path).unwrap();
    let repo = Repository::init(&path).unwrap();
    (path, repo)
}

#[test]
fn list_remotes_returns_sorted_names_and_urls() {
    let (_path, repo) = temp_repo("list");
    repo.remote("zeta", "https://example.com/zeta.git").unwrap();
    repo.remote("alpha", "https://example.com/alpha.git").unwrap();
    repo.remote_set_pushurl("alpha", Some("ssh://example.com/alpha.git")).unwrap();

    let remotes = list_remotes(&repo).unwrap();

    assert_eq!(remotes.len(), 2);
    assert_eq!(remotes[0].name, "alpha");
    assert_eq!(remotes[0].url, "https://example.com/alpha.git");
    assert_eq!(remotes[0].push_url.as_deref(), Some("ssh://example.com/alpha.git"));
    assert_eq!(remotes[1].name, "zeta");
    assert_eq!(remotes[1].url, "https://example.com/zeta.git");
    assert_eq!(remotes[1].push_url, None);
}

#[test]
fn list_remotes_returns_empty_for_repo_without_remotes() {
    let (_path, repo) = temp_repo("empty");

    assert!(list_remotes(&repo).unwrap().is_empty());
}
