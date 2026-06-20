#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Markers {
    Uncommitted,
    Commit,
    Dummy,
}

pub const NONE: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LaneRef {
    pub index: usize,
    pub is_flattened: bool,
}

impl LaneRef {
    pub const fn new(index: usize, is_flattened: bool) -> Self {
        Self { index, is_flattened }
    }
}

// A lane entry points at the commit alias currently occupying that graph lane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub alias: u32,
    pub parent_a: u32,
    pub parent_b: u32,
    pub compressed_parents: Vec<u32>,
    pub marker: Markers,
    pub is_flattened: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk { alias: NONE, parent_a: NONE, parent_b: NONE, compressed_parents: Vec::new(), marker: Markers::Dummy, is_flattened: false }
    }
}

impl Chunk {
    pub fn uncommitted(parent_a: u32, parent_b: u32) -> Self {
        Chunk { alias: NONE, parent_a, parent_b, compressed_parents: Vec::new(), marker: Markers::Uncommitted, is_flattened: false }
    }

    pub fn commit(alias: u32, parent_a: u32, parent_b: u32) -> Self {
        Chunk { alias, parent_a, parent_b, compressed_parents: Vec::new(), marker: Markers::Commit, is_flattened: false }
    }

    pub fn dummy() -> Self {
        Chunk { alias: NONE, parent_a: NONE, parent_b: NONE, compressed_parents: Vec::new(), marker: Markers::Dummy, is_flattened: false }
    }

    pub fn with_flattened(mut self, is_flattened: bool) -> Self {
        self.is_flattened = is_flattened && !self.is_dummy();
        self
    }

    pub fn with_compressed_parents<I>(mut self, parents: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.compressed_parents.clear();
        for parent in parents {
            self.add_compressed_parent(parent);
        }
        self
    }

    pub fn add_compressed_parent(&mut self, parent: u32) {
        if parent == NONE || parent == self.parent_a || parent == self.parent_b || self.compressed_parents.contains(&parent) {
            return;
        }

        self.compressed_parents.push(parent);
    }

    pub fn remove_parent(&mut self, parent: u32) -> bool {
        let mut changed = false;

        if self.parent_a == parent {
            self.parent_a = NONE;
            changed = true;
        }

        if self.parent_b == parent {
            self.parent_b = NONE;
            changed = true;
        }

        let old_len = self.compressed_parents.len();
        self.compressed_parents.retain(|candidate| *candidate != parent);
        changed || self.compressed_parents.len() != old_len
    }

    pub fn has_parent(&self, parent: u32) -> bool {
        parent != NONE && (self.parent_a == parent || self.parent_b == parent || self.compressed_parents.contains(&parent))
    }

    pub fn has_any_parent(&self) -> bool {
        self.parent_a != NONE || self.parent_b != NONE || !self.compressed_parents.is_empty()
    }

    pub fn parent_aliases(&self) -> Vec<u32> {
        let mut parents = Vec::new();
        for parent in [self.parent_a, self.parent_b].into_iter().chain(self.compressed_parents.iter().copied()) {
            if parent != NONE && !parents.contains(&parent) {
                parents.push(parent);
            }
        }
        parents
    }

    pub fn is_dummy(&self) -> bool {
        self.marker == Markers::Dummy
    }
}
