use crate::{
    core::oids::Oids,
    helpers::{colors::ColorPicker, palette::Theme},
};
use im::HashSet;
use ratatui::style::Color;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Branches {
    pub local: HashMap<u32, Vec<String>>,
    pub remote: HashMap<u32, Vec<String>>,
    pub all: HashMap<u32, Vec<String>>,
    pub colors: HashMap<u32, Color>,
    pub sorted: Vec<(u32, String)>,
    pub indices: Vec<usize>,
    pub visible_branch_names: HashSet<String>,
}

impl Branches {
    pub fn feed(&mut self, oids: &Oids, color: &Rc<RefCell<ColorPicker>>, branches_lanes: &HashMap<u32, usize>, branches_local: HashMap<u32, Vec<String>>, branches_remote: HashMap<u32, Vec<String>>) {
        // Replace all derived branch data because a reload may change refs and filters.
        self.local = branches_local;
        self.remote = branches_remote;
        self.all = HashMap::new();
        self.colors = HashMap::new();
        self.sorted = Vec::new();
        self.indices = Vec::new();

        // Merge local and remote refs by alias for graph labels and checkout choices.
        for (&alias, branches) in self.local.iter() {
            self.all.insert(alias, branches.clone());
        }
        for (&oidi, branches) in self.remote.iter() {
            self.all.entry(oidi).and_modify(|existing| existing.extend(branches.iter().cloned())).or_insert_with(|| branches.clone());
        }

        // Keep local refs before remote refs after alphabetical sorting.
        let mut local: Vec<(u32, String)> = self.local.iter().flat_map(|(&alias, branches)| branches.iter().map(move |branch| (alias, branch.clone()))).collect();
        let mut remote: Vec<(u32, String)> = self.remote.iter().flat_map(|(&alias, branches)| branches.iter().map(move |branch| (alias, branch.clone()))).collect();

        // Branch pane order is name-based, independent of graph order.
        local.sort_by(|a, b| a.1.cmp(&b.1));
        remote.sort_by(|a, b| a.1.cmp(&b.1));

        self.sorted = local.into_iter().chain(remote).collect();

        // Branch colors follow the lane where the alias appears in the rendered graph.
        for (oidi, &lane_idx) in branches_lanes.iter() {
            self.colors.insert(*oidi, color.borrow().get_lane(lane_idx));
        }

        // Indices let branch navigation jump to graph rows without scanning on every keypress.
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

    pub fn is_local(&self, branch_name: &String) -> bool {
        self.local.values().any(|branches| branches.iter().any(|current_branch| current_branch.as_str() == branch_name))
    }
}
