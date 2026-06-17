use crate::{
    app::{
        app::{App, Focus},
        state::layout::Layout,
    },
    core::submodules::{SubmoduleEntry, Submodules},
};
use ratatui::{Terminal, backend::TestBackend, layout::Rect};
use std::path::PathBuf;

fn submodule_entry(name: &str) -> SubmoduleEntry {
    SubmoduleEntry {
        name: name.into(),
        path: PathBuf::from(name),
        absolute_path: PathBuf::from(format!("/tmp/{name}")),
        url: None,
        branch: Some("main".into()),
        head: None,
        index: None,
        workdir: None,
        is_open: true,
        is_uninitialized: false,
        is_in_head: true,
        is_in_index: true,
        is_in_config: true,
        is_in_workdir: true,
        is_index_modified: false,
        is_workdir_modified: false,
        has_new_commits: false,
        has_modified_content: false,
        has_untracked_content: false,
    }
}

fn submodules_app(entries: Vec<SubmoduleEntry>) -> App {
    let mut app = App { focus: Focus::Viewport, layout: Layout { submodules: Rect::new(0, 0, 40, 5), submodules_scrollbar: Rect::new(39, 0, 1, 5), ..Default::default() }, ..Default::default() };
    app.layout_config.is_zen = false;
    app.submodules = Submodules::from_entries(entries);
    app
}

#[test]
fn submodules_short_page_stripes_blank_tail_rows() {
    let mut app = submodules_app(vec![submodule_entry("deps/child")]);
    let zebra = app.theme.background_or_default(app.theme.COLOR_GREY_900);

    let backend = TestBackend::new(40, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_submodules(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer[(2, 0)].bg, zebra);
    assert_ne!(buffer[(2, 1)].bg, zebra);
    assert_eq!(buffer[(2, 2)].bg, zebra);
}

#[test]
fn submodules_empty_state_stripes_backdrop() {
    let mut app = submodules_app(Vec::new());
    let zebra = app.theme.background_or_default(app.theme.COLOR_GREY_900);

    let backend = TestBackend::new(40, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_submodules(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer[(2, 0)].bg, zebra);
    assert_ne!(buffer[(2, 1)].bg, zebra);
    assert_eq!(buffer[(2, 2)].bg, zebra);
}
