use super::*;
use ratatui::{Terminal, backend::TestBackend, style::Color};

#[test]
fn default_splash_draw_has_no_reset_backgrounds() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::default();

    terminal.draw(|frame| app.draw(frame)).unwrap();

    let buffer = terminal.backend().buffer();
    assert!(buffer.content().iter().all(|cell| cell.bg != Color::Reset));
}
