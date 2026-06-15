use crate::helpers::palette::Theme;
use im::HashSet;
use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Default)]
pub struct Branches {
    pub local: HashMap<u32, Vec<String>>,
    pub all: HashMap<u32, Vec<String>>,
    pub colors: HashMap<u32, Color>,
    pub sorted: Vec<(u32, String)>,
    pub visible_branch_names: HashSet<String>,
}

impl Branches {
    pub fn get_sorted_aliases(&self) -> &Vec<(u32, String)> {
        &self.sorted
    }

    pub fn get_color(&self, theme: &Theme, branch_alias: &u32) -> Color {
        *self.colors.get(branch_alias).unwrap_or(&theme.COLOR_TEXT)
    }

    pub fn is_local(&self, branch_name: &String) -> bool {
        self.local.values().any(|branches| branches.iter().any(|current_branch| current_branch.as_str() == branch_name))
    }
}
