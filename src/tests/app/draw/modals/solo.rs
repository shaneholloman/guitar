use super::*;
use crate::core::chunk::NONE;
use git2::Oid;
use ratatui::{Terminal, backend::TestBackend, style::Color};

#[test]
fn solo_modal_renders_toggle_prompt_and_highlights_selected_branch() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App { modal_branch_action: BranchModalAction::Toggle, modal_solo_selected: 1, ..Default::default() };
    let alias = app.oids.get_alias_by_oid(Oid::from_bytes(&[7; 20]).unwrap());

    app.graph_selected = 1;
    app.oids.sorted_aliases = vec![NONE, alias];
    app.branches.sorted = vec![(alias, "main".to_string()), (alias, "feature".to_string())];
    app.branches.local.insert(alias, vec!["main".to_string()]);
    app.branches.colors.insert(alias, Color::Yellow);

    terminal.draw(|frame| app.draw_modal_solo(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    let rendered = buffer.content().iter().map(|cell| cell.symbol()).collect::<String>();
    assert!(rendered.contains("select a branch"));
    assert!(rendered.contains("main"));
    assert!(rendered.contains("feature"));

    let feature_row = (0..buffer.area.height)
        .find(|&y| {
            let line = (0..buffer.area.width).map(|x| buffer[(x, y)].symbol()).collect::<String>();
            line.contains("feature")
        })
        .unwrap();
    let feature_col = (0..buffer.area.width).find(|&x| buffer[(x, feature_row)].symbol() == "f").unwrap();

    assert_eq!(buffer[(feature_col, feature_row)].fg, app.theme.COLOR_GRASS);
    assert_eq!(buffer[(feature_col, feature_row)].bg, app.theme.background_color());
}

#[test]
fn solo_modal_highlights_selected_branch_without_lane_color() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App { modal_branch_action: BranchModalAction::Solo, modal_solo_selected: 1, ..Default::default() };
    let alias = app.oids.get_alias_by_oid(Oid::from_bytes(&[8; 20]).unwrap());

    app.graph_selected = 1;
    app.oids.sorted_aliases = vec![NONE, alias];
    app.branches.sorted = vec![(alias, "main".to_string()), (alias, "feature".to_string())];
    app.branches.local.insert(alias, vec!["main".to_string()]);

    terminal.draw(|frame| app.draw_modal_solo(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    let feature_row = (0..buffer.area.height)
        .find(|&y| {
            let line = (0..buffer.area.width).map(|x| buffer[(x, y)].symbol()).collect::<String>();
            line.contains("feature")
        })
        .unwrap();
    let feature_col = (0..buffer.area.width).find(|&x| buffer[(x, feature_row)].symbol() == "f").unwrap();

    assert_eq!(buffer[(feature_col, feature_row)].fg, app.theme.COLOR_GRASS);
    assert_eq!(buffer[(feature_col, feature_row)].bg, app.theme.background_color());
}
