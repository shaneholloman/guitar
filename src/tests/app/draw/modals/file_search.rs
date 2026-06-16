use super::*;
use crate::git::queries::files::FileSearchResult;
use ratatui::{Terminal, backend::TestBackend};

fn rendered_symbols(terminal: &Terminal<TestBackend>) -> String {
    terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>()
}

fn result(path: &str, matched_indices: Vec<usize>) -> FileSearchResult {
    FileSearchResult { path: path.to_string(), score: 1, matched_indices }
}

#[test]
fn file_search_modal_renders_input_suggestions_and_highlighted_matches() {
    let mut app = App { modal_file_search_selected: 0, ..Default::default() };
    app.modal_input.set_value("search");
    app.modal_file_search_results = vec![result("search.rs", vec![0, 1, 2, 3, 4, 5])];

    let backend = TestBackend::new(90, 26);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_modal_file_search(frame, "Search repository files")).unwrap();

    let rendered = rendered_symbols(&terminal);
    assert!(rendered.contains("Search repository files"));
    assert!(rendered.contains("search.rs"));
    assert!(rendered.contains("choose (enter)"));

    let buffer = terminal.backend().buffer();
    let row = (0..buffer.area.height)
        .find(|&y| {
            let line = (0..buffer.area.width).map(|x| buffer[(x, y)].symbol()).collect::<String>();
            line.contains("search.rs")
        })
        .unwrap();
    let col = (0..buffer.area.width).find(|&x| buffer[(x, row)].symbol() == "s").unwrap();

    assert_eq!(buffer[(col, row)].fg, app.theme.COLOR_GRASS);
    assert_eq!(buffer[(col, row)].bg, app.theme.background_or_default(app.theme.COLOR_GREY_800));
}

#[test]
fn file_search_modal_scrolls_long_result_lists() {
    let mut app = App { modal_file_search_selected: 20, ..Default::default() };
    app.modal_input.set_value("file");
    app.modal_file_search_results = (0..30).map(|idx| result(&format!("file_{idx}.rs"), vec![0, 1, 2, 3])).collect();

    let backend = TestBackend::new(90, 26);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_modal_file_search(frame, "Search repository files")).unwrap();

    assert!(app.modal_file_search_scroll.get() > 0);
    assert!(rendered_symbols(&terminal).contains("file_20.rs"));
}

#[test]
fn file_search_modal_renders_empty_and_no_match_states() {
    let mut app = App::default();
    let backend = TestBackend::new(90, 26);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| app.draw_modal_file_search(frame, "Search repository files")).unwrap();
    assert!(rendered_symbols(&terminal).contains("type to search"));

    app.modal_input.set_value("zzzz");
    terminal.draw(|frame| app.draw_modal_file_search(frame, "Search repository files")).unwrap();
    assert!(rendered_symbols(&terminal).contains("no matches"));
}
