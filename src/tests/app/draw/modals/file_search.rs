use super::*;
use crate::git::queries::files::FileSearchResult;
use ratatui::{backend::TestBackend, buffer::Buffer, layout::Rect, Terminal};

fn rendered_symbols(terminal: &Terminal<TestBackend>) -> String {
    terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>()
}

fn result(path: &str, matched_indices: Vec<usize>) -> FileSearchResult {
    FileSearchResult { path: path.to_string(), score: 1, matched_indices }
}

fn row_symbols(buffer: &Buffer, row: u16) -> String {
    (0..buffer.area.width).map(|x| buffer[(x, row)].symbol()).collect::<String>()
}

fn find_row(buffer: &Buffer, text: &str) -> u16 {
    (0..buffer.area.height).find(|&y| row_symbols(buffer, y).contains(text)).unwrap()
}

fn find_col(buffer: &Buffer, row: u16, symbol: &str) -> u16 {
    (0..buffer.area.width).find(|&x| buffer[(x, row)].symbol() == symbol).unwrap()
}

fn file_search_modal_area(area: Rect) -> Rect {
    let modal_width = 76.min((area.width as f32 * 0.85) as usize) as u16;
    let modal_height = 20.min((area.height as f32 * 0.8) as usize) as u16;
    let x = area.x + (area.width.saturating_sub(modal_width)) / 2;
    let y = area.y + (area.height.saturating_sub(modal_height)) / 2;
    Rect::new(x, y, modal_width, modal_height)
}

#[test]
fn file_search_modal_renders_input_suggestions_and_highlighted_matches() {
    let mut app = App { modal_file_search_selected: 0, ..Default::default() };
    app.modal_input.set_value("search");
    app.modal_file_search_results = vec![result("search.rs", vec![0, 1, 2, 3, 4, 5]), result("result.rs", vec![0])];

    let backend = TestBackend::new(90, 26);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_modal_file_search(frame, "Search repository files")).unwrap();

    let rendered = rendered_symbols(&terminal);
    assert!(rendered.contains("Search repository files"));
    assert!(rendered.contains("search.rs"));
    assert!(rendered.contains("result.rs"));
    assert!(rendered.contains("choose (enter)"));

    let buffer = terminal.backend().buffer();
    let selected_row = find_row(buffer, "search.rs");
    let selected_match_col = find_col(buffer, selected_row, "s");
    let selected_text_col = find_col(buffer, selected_row, ".");

    assert_eq!(buffer[(selected_match_col, selected_row)].fg, app.theme.COLOR_GRASS);
    assert_eq!(buffer[(selected_text_col, selected_row)].fg, app.theme.COLOR_HIGHLIGHTED);
    assert_ne!(buffer[(selected_match_col, selected_row)].bg, app.theme.background_or_default(app.theme.COLOR_GREY_800));
    assert_ne!(buffer[(selected_text_col, selected_row)].bg, app.theme.background_or_default(app.theme.COLOR_GREY_800));

    let unselected_row = find_row(buffer, "result.rs");
    let unselected_match_col = find_col(buffer, unselected_row, "r");
    let unselected_text_col = find_col(buffer, unselected_row, "e");

    assert_eq!(buffer[(unselected_match_col, unselected_row)].fg, app.theme.COLOR_GRASS);
    assert_eq!(buffer[(unselected_text_col, unselected_row)].fg, app.theme.COLOR_TEXT);
}

#[test]
fn file_search_modal_stretches_input_borders_and_leaves_gap_before_suggestions() {
    let mut app = App { modal_file_search_selected: 0, ..Default::default() };
    app.modal_input.set_value("search");
    app.modal_file_search_results = vec![result("search.rs", vec![0, 1, 2, 3, 4, 5])];

    let backend = TestBackend::new(90, 26);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_modal_file_search(frame, "Search repository files")).unwrap();

    let buffer = terminal.backend().buffer();
    let modal_area = file_search_modal_area(buffer.area);
    let input_top = modal_area.y + 4;
    let input_bottom = input_top + 4;
    let first_suggestion_row = find_row(buffer, "search.rs");

    assert_eq!(first_suggestion_row, input_bottom + 2);
    for row in [input_top, input_bottom] {
        for x in modal_area.x + 1..modal_area.x + modal_area.width - 1 {
            assert_eq!(buffer[(x, row)].symbol(), "─");
        }
    }
    for x in modal_area.x + 1..modal_area.x + modal_area.width - 1 {
        assert_eq!(buffer[(x, input_bottom + 1)].symbol(), " ");
    }
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
