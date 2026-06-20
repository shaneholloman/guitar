use crate::{core::chunk::LaneRef, helpers::palette::*};
use ratatui::style::Color;

#[derive(Clone)]
pub struct ColorPicker {
    palette_a: [Color; 16],
    flattened_lane: Color,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::from_theme(&Theme::default())
    }
}

impl ColorPicker {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            palette_a: [
                theme.COLOR_GRASS,
                theme.COLOR_GREEN,
                theme.COLOR_CYAN,
                theme.COLOR_TEAL,
                theme.COLOR_INDIGO,
                theme.COLOR_BLUE,
                theme.COLOR_PURPLE,
                theme.COLOR_DURPLE,
                theme.COLOR_RED,
                theme.COLOR_PINK,
                theme.COLOR_GRAPEFRUIT,
                theme.COLOR_BROWN,
                theme.COLOR_AMBER,
                theme.COLOR_ORANGE,
                theme.COLOR_LIME,
                theme.COLOR_YELLOW,
            ],
            flattened_lane: theme.COLOR_GREY_500,
        }
    }

    pub fn get_lane(&self, lane: usize) -> Color {
        self.palette_a[lane % self.palette_a.len()]
    }

    pub fn get_lane_ref(&self, lane: LaneRef) -> Color {
        if lane.is_flattened { self.flattened_lane } else { self.get_lane(lane.index) }
    }
}
