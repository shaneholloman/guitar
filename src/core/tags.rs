use crate::helpers::palette::Theme;
use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Default)]
pub struct Tags {
    pub local: HashMap<u32, Vec<String>>,
    pub colors: HashMap<u32, Color>,
    pub sorted: Vec<(u32, String)>,
}

impl Tags {
    pub fn get_sorted_aliases(&self) -> &Vec<(u32, String)> {
        &self.sorted
    }

    pub fn get_color(&self, theme: &Theme, branch_alias: &u32) -> Color {
        *self.colors.get(branch_alias).unwrap_or(&theme.COLOR_TEXT)
    }
}
