use super::*;
use crate::core::chunk::NONE;
use crate::core::reflogs::HeadReflogAliasEntry;
use git2::Signature;
use std::{
    fs,
    path::Path,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_repo(name: &str) -> (std::path::PathBuf, Repository) {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("guitar-input-git-{name}-{id}"));
    fs::create_dir_all(&path).unwrap();
    let repo = Repository::init(&path).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
    }
    (path, repo)
}

fn commit(repo: &Repository, file: &str, message: &str) -> git2::Oid {
    let workdir = repo.workdir().unwrap().to_path_buf();
    fs::write(workdir.join(file), "content\n").unwrap();

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
fn cherrypick_opens_message_modal_with_prefilled_summary() {
    let (_path, repo) = temp_repo("cherrypick-modal");
    let oid = commit(&repo, "file.txt", "original summary\n\nbody");

    let mut app = App { repo: Some(Rc::new(repo)), viewport: Viewport::Graph, focus: Focus::Viewport, graph_selected: 1, ..Default::default() };
    let alias = app.oids.get_alias_by_oid(oid);
    app.oids.sorted_aliases = vec![NONE, alias];

    app.on_cherrypick();

    assert_eq!(app.focus, Focus::ModalCherrypick);
    assert_eq!(app.pending_cherrypick_oid, Some(oid));
    assert_eq!(app.modal_input.value(), "cherrypicked: original summary");
}

#[test]
fn create_branch_from_reflog_uses_reflog_commit_target() {
    let (_path, repo) = temp_repo("reflog-branch-target");
    let graph_oid = commit(&repo, "graph.txt", "graph");
    let reflog_oid = commit(&repo, "reflog.txt", "reflog");

    let mut app = App { repo: Some(Rc::new(repo)), viewport: Viewport::Graph, focus: Focus::Reflogs, graph_selected: 1, ..Default::default() };
    let graph_alias = app.oids.get_alias_by_oid(graph_oid);
    let reflog_alias = app.oids.get_alias_by_oid(reflog_oid);
    app.oids.sorted_aliases = vec![NONE, graph_alias, reflog_alias];
    app.reflogs.entries.push(HeadReflogAliasEntry {
        selector: "HEAD@{0}".to_string(),
        old_oid: graph_oid,
        new_oid: reflog_oid,
        new_alias: reflog_alias,
        message: "commit: reflog".to_string(),
        time: git2::Time::new(1, 0),
    });

    app.on_create_branch();

    assert_eq!(app.focus, Focus::ModalCreateBranch);
    assert_eq!(app.selected_branch_target_oid(), Some(reflog_oid));
}
