use crate::helpers::palette::Theme;
use crate::{core::oids::Oids, helpers::colors::ColorPicker};
use ratatui::style::Color;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Tags {
    pub local: HashMap<u32, Vec<String>>,
    pub colors: HashMap<u32, Color>,
    pub sorted: Vec<(u32, String)>,
    pub indices: Vec<usize>,
}

impl Tags {
    pub fn feed(&mut self, oids: &Oids, color: &Rc<RefCell<ColorPicker>>, tags_lanes: &HashMap<u32, usize>, tags_local: HashMap<u32, Vec<String>>) {
        // Replace all derived tag data because tags may move or disappear after actions.
        self.local = tags_local;
        self.colors = HashMap::new();
        self.sorted = Vec::new();
        self.indices = Vec::new();

        // Flatten the alias-to-tags map into the tag pane row model.
        let sorted: Vec<(u32, String)> = self.local.iter().flat_map(|(&alias, tags)| tags.iter().map(move |tag| (alias, tag.clone()))).collect();
        self.sorted = sorted;

        // Tag pane order is name-based, independent of graph order.
        self.sorted.sort_by(|a, b| a.1.cmp(&b.1));

        // Tag colors follow the lane where the tagged commit appears.
        for (oidi, &lane_idx) in tags_lanes.iter() {
            self.colors.insert(*oidi, color.borrow().get_lane(lane_idx));
        }

        // Indices let tag navigation jump to graph rows without recomputing positions.
        let mut sorted_time = self.sorted.clone();
        let index_map: std::collections::HashMap<u32, usize> = oids.get_sorted_aliases().iter().enumerate().map(|(i, &oidi)| (oidi, i)).collect();

        sorted_time.sort_by_key(|(oidi, _)| index_map.get(oidi).copied().unwrap_or(usize::MAX));
        self.indices = Vec::new();
        sorted_time.iter().for_each(|(oidi, _)| {
            self.indices.push(oids.get_sorted_aliases().iter().position(|o| oidi == o).unwrap_or(usize::MAX));
        });
    }

    pub fn get_sorted_aliases(&self) -> &Vec<(u32, String)> {
        &self.sorted
    }

    pub fn get_color(&self, theme: &Theme, branch_alias: &u32) -> Color {
        *self.colors.get(branch_alias).unwrap_or(&theme.COLOR_TEXT)
    }
}
