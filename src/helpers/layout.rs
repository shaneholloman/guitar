use facet::Facet;
use ratatui::layout::Rect;
use std::{fs, path::PathBuf};

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

fn layout_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("guitar");
    path.push("layout.json");
    path
}

#[derive(Facet, Default, Clone, Copy)]
pub struct LayoutConfig {
    pub is_shas: bool,
    pub is_minimal: bool,
    pub is_branches: bool,
    pub is_tags: bool,
    pub is_stashes: bool,
    pub is_status: bool,
    pub is_inspector: bool,
    pub is_zen: bool,
}

pub fn load_layout_config() -> LayoutConfig {
    let path = layout_path();
    if path.exists() {
        let contents = fs::read_to_string(&path).unwrap();
        facet_json::from_str(&contents).unwrap_or_default()
    } else {
        let config = LayoutConfig::default();
        save_layout_config(&config);
        config
    }
}

pub fn save_layout_config(config: &LayoutConfig) {
    let path = layout_path();
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        let _ = fs::create_dir_all(parent);
    }

    let config_string = facet_json::to_string(config).unwrap();
    fs::write(&path, &config_string).unwrap();
}
