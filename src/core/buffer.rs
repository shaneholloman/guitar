use crate::core::chunk::{Chunk, LaneRef, NONE};
use im::{OrdMap, Vector};

#[derive(Default, Clone)]
pub struct Delta {
    pub ops: Vec<DeltaOp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeltaOp {
    Insert { index: usize, item: Chunk },
    Remove { index: usize },
    Replace { index: usize, new: Chunk },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpdateOutcome {
    pub lane: LaneRef,
    pub started_lane: bool,
}

#[derive(Default, Clone)]
pub struct Buffer {
    pub curr: Vector<Chunk>,
    // Deltas keep memory bounded while still allowing visible ranges to be reconstructed.
    pub deltas: Vector<Delta>,
    pub checkpoints: OrdMap<usize, Vector<Chunk>>,
    pub delta: Delta,
    mergers: Vec<u32>,
    transient_lanes: Vec<usize>,
    lane_limit: Option<usize>,
}

impl Buffer {
    pub fn with_lane_limit(limit: usize) -> Self {
        Self { lane_limit: Some(limit.max(1)), ..Self::default() }
    }

    pub fn merger(&mut self, alias: u32) {
        self.mergers.push(alias);
    }

    pub fn expire_lane_after_snapshot(&mut self, lane_idx: usize) {
        if let Some(limit) = self.lane_limit {
            if lane_idx >= limit {
                return;
            }
            if lane_idx + 1 == limit && self.curr.get(lane_idx).is_some_and(|chunk| chunk.is_flattened) {
                return;
            }
        }

        if !self.transient_lanes.iter().any(|idx| *idx == lane_idx) {
            self.transient_lanes.push(lane_idx);
        }
    }

    pub fn update(&mut self, chunk: Chunk) -> UpdateOutcome {
        self.backup();

        let transient_lanes = std::mem::take(&mut self.transient_lanes);
        for lane_idx in transient_lanes {
            if lane_idx < self.curr.len() && !self.curr[lane_idx].is_dummy() {
                self.curr[lane_idx] = Chunk::dummy();
                self.delta.ops.push(DeltaOp::Replace { index: lane_idx, new: self.curr[lane_idx].clone() });
            }
        }

        // Trailing dummy lanes carry no future topology and can be removed immediately.
        while let Some(last_idx) = self.curr.len().checked_sub(1) {
            if !self.curr[last_idx].is_dummy() {
                break;
            }
            self.curr.pop_back();
            self.delta.ops.push(DeltaOp::Remove { index: last_idx });
        }

        // Planned mergers split a lane so the second parent can draw toward its target later.
        if let Some(merger_idx) = self.curr.iter().position(|inner| self.mergers.iter().any(|alias| *alias == inner.alias)) {
            if let Some(merger_pos) = self.mergers.iter().position(|alias| *alias == self.curr[merger_idx].alias) {
                self.mergers.remove(merger_pos);
            }

            let mut clone = self.curr[merger_idx].clone();
            clone.parent_a = clone.parent_b;
            clone.parent_b = NONE;
            clone.compressed_parents.clear();
            self.curr[merger_idx].parent_b = NONE;
            self.curr.push_back(clone.clone());

            self.delta.ops.push(DeltaOp::Replace { index: merger_idx, new: self.curr[merger_idx].clone() });

            self.delta.ops.push(DeltaOp::Insert { index: self.curr.len() - 1, item: clone });
        }

        // Prefer replacing the parent lane; append only when the commit starts a new lane.
        if let Some(first_idx) = self.curr.iter().position(|inner| inner.parent_a == chunk.alias || inner.compressed_parents.contains(&chunk.alias)) {
            let old_alias = chunk.alias;
            let replacement = self.replacement_chunk_for_lane(first_idx, chunk);

            self.curr[first_idx] = replacement.clone();
            self.delta.ops.push(DeltaOp::Replace { index: first_idx, new: replacement });

            // Clear consumed parent pointers so inactive branch lanes collapse into dummies.
            for (i, inner) in self.curr.iter_mut().enumerate() {
                if inner.alias == old_alias {
                    continue;
                }

                let parents_changed = inner.remove_parent(old_alias);

                if parents_changed {
                    if !inner.has_any_parent() {
                        *inner = Chunk::dummy();
                    }

                    self.delta.ops.push(DeltaOp::Replace { index: i, new: inner.clone() });
                }
            }

            self.enforce_lane_limit(Some(first_idx));
            UpdateOutcome { lane: self.lane_ref_for_original_index(first_idx), started_lane: false }
        } else {
            self.curr.push_back(chunk.clone());
            self.delta.ops.push(DeltaOp::Insert { index: self.curr.len() - 1, item: chunk });
            let lane_idx = self.curr.len() - 1;
            self.enforce_lane_limit(Some(lane_idx));
            UpdateOutcome { lane: self.lane_ref_for_original_index(lane_idx), started_lane: true }
        }
    }

    fn enforce_lane_limit(&mut self, preferred_idx: Option<usize>) {
        let Some(limit) = self.lane_limit else {
            return;
        };

        if self.curr.len() <= limit {
            self.purge_unstored_mergers();
            self.transient_lanes.retain(|lane_idx| *lane_idx < limit);
            return;
        }

        let cap_idx = limit - 1;
        let replacement = self.flattened_representative(cap_idx, preferred_idx);
        if self.curr[cap_idx] != replacement {
            self.curr[cap_idx] = replacement.clone();
            self.delta.ops.push(DeltaOp::Replace { index: cap_idx, new: replacement });
        }

        while self.curr.len() > limit {
            let idx = self.curr.len() - 1;
            self.curr.pop_back();
            self.delta.ops.push(DeltaOp::Remove { index: idx });
        }

        self.purge_unstored_mergers();
        self.transient_lanes.retain(|lane_idx| *lane_idx < limit && (*lane_idx + 1 != limit || !self.curr.get(*lane_idx).is_some_and(|chunk| chunk.is_flattened)));
    }

    fn flattened_representative(&self, cap_idx: usize, preferred_idx: Option<usize>) -> Chunk {
        let preferred = preferred_idx.and_then(|idx| (idx >= cap_idx).then(|| self.curr.get(idx)).flatten()).filter(|chunk| !chunk.is_dummy()).cloned();

        let fallback = self.curr.iter().skip(cap_idx).find(|chunk| !chunk.is_dummy()).cloned().unwrap_or_else(Chunk::dummy);
        let mut representative = preferred.unwrap_or(fallback).with_flattened(true);
        for parent in self.curr.iter().skip(cap_idx).filter(|chunk| !chunk.is_dummy()).flat_map(Chunk::parent_aliases) {
            representative.add_compressed_parent(parent);
        }
        representative
    }

    fn replacement_chunk_for_lane(&self, lane_idx: usize, mut replacement: Chunk) -> Chunk {
        let old = &self.curr[lane_idx];
        if !old.is_flattened {
            return replacement;
        }

        replacement = replacement.with_flattened(true);
        for parent in old.parent_aliases() {
            if parent != replacement.alias {
                replacement.add_compressed_parent(parent);
            }
        }
        replacement
    }

    fn lane_ref_for_original_index(&self, lane_idx: usize) -> LaneRef {
        if let Some(limit) = self.lane_limit
            && lane_idx >= limit
        {
            return LaneRef::new(limit - 1, true);
        }

        LaneRef::new(lane_idx, self.curr.get(lane_idx).is_some_and(|chunk| chunk.is_flattened))
    }

    fn purge_unstored_mergers(&mut self) {
        self.mergers.retain(|alias| self.curr.iter().any(|chunk| !chunk.is_dummy() && chunk.alias == *alias));
    }

    pub fn backup(&mut self) {
        let old = std::mem::take(&mut self.delta);
        self.deltas.push_back(old);
        let idx = self.deltas.len().saturating_sub(1);
        if idx.is_multiple_of(100) {
            self.checkpoints.insert(idx, self.curr.clone());
        }
    }

    pub fn window(&self, start: usize, end: usize) -> Vector<Vector<Chunk>> {
        let mut history = Vector::new();

        // Start from the nearest checkpoint before the requested range.
        let checkpoint_idx = self.checkpoints.keys().rev().find(|&&idx| idx <= start).copied();

        let mut curr = checkpoint_idx.and_then(|idx| self.checkpoints.get(&idx)).cloned().unwrap_or_default();

        // Replay only the deltas needed to produce the requested visible range.
        let begin = checkpoint_idx.map_or(0, |idx| idx + 1);
        let end = end.min(self.deltas.len());

        for delta in self.deltas.iter().skip(begin).take(end - begin) {
            for op in delta.ops.iter() {
                match op {
                    DeltaOp::Insert { index, item } => {
                        curr.insert(*index, item.clone());
                    },
                    DeltaOp::Remove { index } => {
                        curr.remove(*index);
                    },
                    DeltaOp::Replace { index, new } => {
                        curr[*index] = new.clone();
                    },
                }
            }
            history.push_back(curr.clone());
        }

        history
    }
}

#[cfg(test)]
#[path = "../tests/core/buffer.rs"]
mod tests;
