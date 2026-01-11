use ratatui::layout::Rect;

pub const LAYOUT_WIDTH_LEFT_PANE: u16 = 45;
pub const LAYOUT_WIDTH_RIGHT_PANE: u16 = 46;
pub const LAYOUT_WIDTH_MIN_CENTER: u16 = 20;
pub const LAYOUT_PERCENTAGE_LEFT_PANE_CRAMPED: u16 = 30;
pub const LAYOUT_PERCENTAGE_CENTER_PANE_CRAMPED: u16 = 40;
pub const LAYOUT_PERCENTAGE_RIGHT_PANE_CRAMPED: u16 = 30;

pub fn inset_top(mut r: Rect, n: u16) -> Rect {
    r.y += n;
    r.height = r.height.saturating_sub(n);
    r
}

pub fn inset_bottom(mut r: Rect, n: u16) -> Rect {
    r.height = r.height.saturating_sub(n);
    r
}

pub fn add_scrollbar(mut r: Rect) -> Rect {
    r.width += 1;
    r
}

pub fn extend_up(mut r: Rect, n: u16) -> Rect {
    r.y = r.y.saturating_sub(n);
    r.height += n;
    r
}

pub fn shrink_width(mut r: Rect, n: u16) -> Rect {
    r.width = r.width.saturating_sub(n);
    r
}
