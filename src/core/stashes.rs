use crate::helpers::colors::ColorPicker;
use ratatui::style::Color;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Stashes {
    pub colors: HashMap<u32, Color>,
}

impl Stashes {
    pub fn feed(&mut self, color: &Rc<RefCell<ColorPicker>>, stashes_lanes: &HashMap<u32, usize>) {
        // Rebuild colors because stash lanes can shift as history loads.
        self.colors = HashMap::new();

        // Stash colors follow the lane where the synthetic stash row appears.
        for (oidi, &lane_idx) in stashes_lanes.iter() {
            self.colors.insert(*oidi, color.borrow().get_lane(lane_idx));
        }
    }
}
