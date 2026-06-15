use super::*;
use ratatui::{Terminal, backend::TestBackend, layout::Rect, style::Modifier, widgets::Paragraph};

#[test]
fn deferred_surface_replays_last_ready_front_buffer() {
    let backend = TestBackend::new(12, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::default();

    terminal
        .draw(|frame| {
            app.draw_surface(frame, DrawSurface::Graph, |_app, surface| {
                surface.render_widget(Paragraph::new("ready"), Rect::new(0, 0, 5, 1));
            });
        })
        .unwrap();

    terminal
        .draw(|frame| {
            app.draw_surface(frame, DrawSurface::Graph, |_app, surface| {
                surface.render_widget(Paragraph::new("wait"), Rect::new(0, 0, 4, 1));
                SurfaceRender::Deferred
            });
        })
        .unwrap();

    let rendered = terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>();
    assert!(rendered.contains("ready"));
    assert!(!rendered.contains("wait"));

    let buffer = terminal.backend().buffer();
    assert!(!buffer[(0, 0)].modifier.contains(Modifier::DIM));
}
