use super::*;
use crate::app::app::Viewport;
use git2::{Repository, Signature};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_repo(name: &str) -> (PathBuf, Repository) {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("guitar-input-modals-{name}-{id}"));
    fs::create_dir_all(&path).unwrap();
    let repo = Repository::init(&path).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
    }
    (path, repo)
}

fn write_file(root: &Path, file: &str) {
    let path = root.join(file);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, "content\n").unwrap();
}

fn commit_files(repo: &Repository, files: &[&str], message: &str) {
    let workdir = repo.workdir().unwrap().to_path_buf();
    for file in files {
        write_file(&workdir, file);
    }

    let mut index = repo.index().unwrap();
    for file in files {
        index.add_path(Path::new(file)).unwrap();
    }
    index.write().unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();
    let sig = Signature::now("Test User", "test@example.com").unwrap();
    let parent = repo.head().ok().and_then(|head| head.peel_to_commit().ok());
    let parents: Vec<&git2::Commit<'_>> = parent.iter().collect();
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents).unwrap();
}

fn modal_app(name: &str) -> App {
    let (_path, repo) = temp_repo(name);
    commit_files(&repo, &["src/app/draw/search.rs", "src/app/draw/status.rs", "src/app/draw/stashes.rs"], "files");
    App { repo: Some(Rc::new(repo)), viewport: Viewport::Graph, focus: Focus::ModalFileSearch, modal_file_search_return_focus: Focus::Branches, ..Default::default() }
}

fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent::new(code, modifiers)
}

#[test]
fn file_search_esc_closes_and_restores_prior_focus() {
    let (_path, repo) = temp_repo("esc");
    let mut app = App { repo: Some(Rc::new(repo)), viewport: Viewport::Graph, focus: Focus::Reflogs, ..Default::default() };
    app.on_find_file();
    app.modal_input.set_value("search");
    app.modal_file_search_results.push(crate::git::queries::files::FileSearchResult { path: "src/app/draw/search.rs".to_string(), score: 1, matched_indices: vec![13] });

    app.handle_modal_key_event(key(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(app.focus, Focus::Reflogs);
    assert!(app.modal_input.value().is_empty());
    assert!(app.modal_file_search_results.is_empty());
}

#[test]
fn file_search_typing_updates_results_and_resets_selection() {
    let mut app = modal_app("typing");
    app.modal_file_search_selected = 2;
    app.modal_file_search_scroll.set(2);

    app.handle_modal_key_event(key(KeyCode::Char('s'), KeyModifiers::NONE));

    assert_eq!(app.modal_input.value(), "s");
    assert!(!app.modal_file_search_results.is_empty());
    assert_eq!(app.modal_file_search_selected, 0);
    assert_eq!(app.modal_file_search_scroll.get(), 0);
}

#[test]
fn file_search_ctrl_and_arrow_keys_move_selection() {
    let mut app = modal_app("navigation");
    app.handle_modal_key_event(key(KeyCode::Char('s'), KeyModifiers::NONE));
    assert!(app.modal_file_search_results.len() > 1);

    app.handle_modal_key_event(key(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.modal_file_search_selected, 1);

    app.handle_modal_key_event(key(KeyCode::Char('k'), KeyModifiers::CONTROL));
    assert_eq!(app.modal_file_search_selected, 0);

    app.handle_modal_key_event(key(KeyCode::Char('j'), KeyModifiers::CONTROL));
    assert_eq!(app.modal_file_search_selected, 1);

    app.handle_modal_key_event(key(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.modal_file_search_selected, 0);
}

#[test]
fn file_search_enter_fills_input_without_opening_viewer() {
    let mut app = modal_app("enter");
    for ch in "search".chars() {
        app.handle_modal_key_event(key(KeyCode::Char(ch), KeyModifiers::NONE));
    }

    app.handle_modal_key_event(key(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.modal_input.value(), "src/app/draw/search.rs");
    assert_eq!(app.focus, Focus::ModalFileSearch);
    assert_eq!(app.viewport, Viewport::Graph);
}

#[test]
fn file_search_plain_l_is_inserted_into_input() {
    let mut app = modal_app("plain-l");

    app.handle_modal_key_event(key(KeyCode::Char('l'), KeyModifiers::NONE));

    assert_eq!(app.modal_input.value(), "l");
}
