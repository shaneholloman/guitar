use crate::{core::oids::Oids, git::queries::reflogs::HeadReflogEntry, helpers::colors::ColorPicker};
use git2::{Oid, Time};
use ratatui::style::Color;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadReflogAliasEntry {
    pub selector: String,
    pub old_oid: Oid,
    pub new_oid: Oid,
    pub new_alias: u32,
    pub message: String,
    pub time: Time,
}

#[derive(Default)]
pub struct HeadReflogs {
    pub entries: Vec<HeadReflogAliasEntry>,
    pub latest_by_alias: HashMap<u32, HeadReflogAliasEntry>,
    pub colors: HashMap<u32, Color>,
    pub indices: Vec<usize>,
}

impl HeadReflogs {
    pub fn feed(&mut self, oids: &Oids, color: &Rc<RefCell<ColorPicker>>, reflog_lanes: &HashMap<u32, usize>, entries: Vec<HeadReflogEntry>) {
        self.entries = Vec::new();
        self.latest_by_alias = HashMap::new();
        self.colors = HashMap::new();
        self.indices = Vec::new();

        let index_map: HashMap<u32, usize> = oids.get_sorted_aliases().iter().enumerate().map(|(idx, &alias)| (alias, idx)).collect();

        for entry in entries {
            let Some(&new_alias) = oids.aliases.get(&entry.new_oid) else {
                continue;
            };
            let alias_entry = HeadReflogAliasEntry { selector: entry.selector, old_oid: entry.old_oid, new_oid: entry.new_oid, new_alias, message: entry.message, time: entry.time };

            self.latest_by_alias.entry(new_alias).or_insert_with(|| alias_entry.clone());
            self.indices.push(index_map.get(&new_alias).copied().unwrap_or(usize::MAX));
            self.entries.push(alias_entry);
        }

        for (alias, &lane_idx) in reflog_lanes {
            self.colors.insert(*alias, color.borrow().get_lane(lane_idx));
        }
    }

    pub fn latest_for_alias(&self, alias: u32) -> Option<&HeadReflogAliasEntry> {
        self.latest_by_alias.get(&alias)
    }

    pub fn get_color(&self, alias: u32) -> Option<Color> {
        self.colors.get(&alias).copied()
    }
}

#[cfg(test)]
#[path = "../tests/core/reflogs.rs"]
mod tests;
