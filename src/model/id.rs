use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerId(u64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdGen {
    next_id: Id,
}

impl IdGen {
    pub fn new() -> Self {
        Self { next_id: Id(0) }
    }

    pub fn next_player(&mut self) -> PlayerId {
        let Id(id) = self.next();
        PlayerId(id)
    }

    pub fn next(&mut self) -> Id {
        let id = self.next_id;
        self.next_id.0 += 1;
        id
    }
}

impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}
