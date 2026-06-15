use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Default)]
pub struct Stashes {
    pub colors: HashMap<u32, Color>,
}
