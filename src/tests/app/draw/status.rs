use super::*;
use crate::{
    app::state::layout::Layout,
    core::graph_service::{GraphIndexIdentity, GraphRow},
    git::queries::helpers::{FileChange, FileChanges},
};
use git2::Oid;
use ratatui::{Terminal, backend::TestBackend, layout::Rect};

fn status_app() -> App {
    let mut app = App {
        layout: Layout {
            status_top: Rect::new(0, 0, 48, 5),
            status_top_scrollbar: Rect::new(47, 0, 1, 5),
            status_bottom: Rect::new(0, 5, 48, 5),
            status_bottom_scrollbar: Rect::new(47, 5, 1, 5),
            ..Default::default()
        },
        ..Default::default()
    };
    app.layout_config.is_zen = false;
    app.layout_config.is_inspector = false;
    app
}

fn rendered(terminal: &Terminal<TestBackend>) -> String {
    terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>()
}

fn graph_row(index: usize, alias: u32, oid: Oid) -> GraphRow {
    GraphRow {
        index,
        alias,
        oid,
        summary: "summary".to_string(),
        has_any_branch: false,
        branches: Vec::new(),
        tags: Vec::new(),
        is_stash: false,
        stash_lane: None,
        worktrees: Vec::new(),
        reflog: None,
    }
}

#[test]
fn status_shows_loading_instead_of_stale_commit_diff() {
    let mut app = status_app();
    app.graph_selected = 1;
    app.current_diff = vec![FileChange { filename: "stale.txt".to_string(), status: FileStatus::Modified }];
    app.current_diff_identity = None;

    let backend = TestBackend::new(48, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_status(frame)).unwrap();

    let rendered = rendered(&terminal);
    assert!(rendered.contains("loading"), "{rendered}");
    assert!(!rendered.contains("stale.txt"), "{rendered}");
}

#[test]
fn status_preserves_known_empty_commit_diff_state() {
    let mut app = status_app();
    let identity = GraphIndexIdentity { index: 1, alias: 1, oid: Oid::zero() };
    app.graph_selected = 1;
    app.graph.index_rows.insert(1, graph_row(1, 1, identity.oid));
    app.current_diff_identity = Some(identity);

    let backend = TestBackend::new(48, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_status(frame)).unwrap();

    let rendered = rendered(&terminal);
    assert!(rendered.contains("⊘ no staged changes"), "{rendered}");
    assert!(!rendered.contains("loading"), "{rendered}");
}

#[test]
fn staged_status_short_page_stripes_blank_tail_rows() {
    let mut app = status_app();
    app.graph_selected = 0;
    app.is_uncommitted_loaded = true;
    app.focus = Focus::Viewport;
    app.uncommitted.staged = FileChanges { modified: vec!["staged.txt".to_string()], ..Default::default() };
    let zebra = app.theme.background_or_default(app.theme.COLOR_GREY_900);

    let backend = TestBackend::new(48, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_status(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer[(1, 0)].bg, zebra);
    assert_ne!(buffer[(1, 1)].bg, zebra);
    assert_eq!(buffer[(1, 2)].bg, zebra);
}

#[test]
fn unstaged_status_short_page_stripes_blank_tail_rows() {
    let mut app = status_app();
    app.graph_selected = 0;
    app.is_uncommitted_loaded = true;
    app.focus = Focus::Viewport;
    app.uncommitted.unstaged = FileChanges { modified: vec!["unstaged.txt".to_string()], ..Default::default() };
    let zebra = app.theme.background_or_default(app.theme.COLOR_GREY_900);

    let backend = TestBackend::new(48, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_status(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer[(1, 6)].bg, zebra);
    assert_ne!(buffer[(1, 7)].bg, zebra);
    assert_eq!(buffer[(1, 8)].bg, zebra);
}
