use crate::{
    app::{
        app::{App, Focus},
        state::layout::Layout,
    },
    core::branches::Branches,
};
use ratatui::{Terminal, backend::TestBackend, layout::Rect};

fn rendered_branches(app: &mut App) -> String {
    let backend = TestBackend::new(30, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            app.draw_branches(frame);
        })
        .unwrap();
    terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>()
}

#[test]
fn branches_render_hidden_local_rows_with_hollow_icon() {
    let mut branches = Branches { sorted: vec![(1, "feature".to_string()), (2, "main".to_string())], ..Default::default() };
    branches.local.insert(1, vec!["feature".to_string()]);
    branches.local.insert(2, vec!["main".to_string()]);
    branches.hidden_branch_names.insert("feature".to_string());

    let mut app = App { focus: Focus::Branches, branches, layout: Layout { branches: Rect::new(0, 0, 30, 5), branches_scrollbar: Rect::new(29, 0, 1, 5), ..Default::default() }, ..Default::default() };

    let rendered = rendered_branches(&mut app);

    assert!(rendered.contains("○ feature"));
    assert!(rendered.contains("● main"));
}
