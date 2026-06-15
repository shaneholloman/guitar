use super::*;
use git2::{Oid, Repository, Signature};
use im::HashSet;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool, mpsc::channel},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn temp_repo(name: &str) -> (PathBuf, Repository) {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("guitar-graph-service-{name}-{id}"));
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
fn graph_service_reports_progress_and_answers_visible_window() {
    let (path, repo) = temp_repo("window");
    commit(&repo, "one.txt", "one");
    let two = commit(&repo, "two.txt", "two");

    let generation = 42;
    let (cmd_tx, cmd_rx) = channel();
    let (event_tx, event_rx) = channel();
    let cancel = Arc::new(AtomicBool::new(false));
    let handle = spawn_graph_service(
        GraphServiceConfig { generation, path: path.display().to_string(), amount: 1, visible_branch_names: HashSet::new(), include_head_reflog_roots: false, worktrees: Vec::new() },
        cmd_rx,
        event_tx,
        cancel.clone(),
    );

    let mut saw_progress = false;
    for _ in 0..20 {
        match event_rx.recv_timeout(Duration::from_millis(250)).unwrap() {
            GraphEvent::Progress { generation: event_generation, total, .. } if event_generation == generation => {
                saw_progress = true;
                assert!(total > 0);
                break;
            },
            _ => {},
        }
    }
    assert!(saw_progress);

    cmd_tx.send(GraphCommand::QueryGraphWindow { generation, request_id: 7, start: 0, end: 2 }).unwrap();

    let mut saw_window = false;
    for _ in 0..20 {
        match event_rx.recv_timeout(Duration::from_millis(250)).unwrap() {
            GraphEvent::GraphWindow { generation: event_generation, request_id, rows, history, total, .. } if event_generation == generation && request_id == 7 => {
                saw_window = true;
                assert!(total >= rows.len());
                assert!(!rows.is_empty());
                assert!(!history.is_empty());
                assert!(rows.len() <= 2);
                break;
            },
            _ => {},
        }
    }
    assert!(saw_window);

    cmd_tx.send(GraphCommand::Lookup { generation, request_id: 8, kind: GraphLookupKind::ShaPrefix { prefix: two.to_string()[..8].to_string() } }).unwrap();

    let mut saw_lookup = false;
    for _ in 0..20 {
        match event_rx.recv_timeout(Duration::from_millis(250)).unwrap() {
            GraphEvent::LookupResult { generation: event_generation, request_id, result: GraphLookupResult::Index(Some(_)), .. } if event_generation == generation && request_id == 8 => {
                saw_lookup = true;
                break;
            },
            _ => {},
        }
    }
    assert!(saw_lookup);

    cmd_tx.send(GraphCommand::Lookup { generation, request_id: 9, kind: GraphLookupKind::Oid { oid: two } }).unwrap();

    let mut saw_oid_lookup = false;
    for _ in 0..20 {
        match event_rx.recv_timeout(Duration::from_millis(250)).unwrap() {
            GraphEvent::LookupResult { generation: event_generation, request_id, result: GraphLookupResult::Index(Some(index)), .. } if event_generation == generation && request_id == 9 => {
                saw_oid_lookup = true;
                assert_eq!(index, 1);
                break;
            },
            _ => {},
        }
    }
    assert!(saw_oid_lookup);

    let _ = cmd_tx.send(GraphCommand::Shutdown);
    cancel.store(true, std::sync::atomic::Ordering::SeqCst);
    handle.join().unwrap();
}
