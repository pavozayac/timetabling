use crate::utils::has_unique_items;

use super::events::EventInstance;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Slot {
    id: i64,
}

impl Slot {
    pub fn new(id: i64) -> Slot {
        Slot { id }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Outline {
    pub slots: Vec<Slot>,
}

impl Outline {
    pub fn new() -> Outline {
        Outline { slots: vec![] }
    }

    pub fn slots_slice(&self) -> &[Slot] {
        &self.slots
    }
}

impl From<Vec<Slot>> for Outline {
    fn from(value: Vec<Slot>) -> Self {
        Outline { slots: value }
    }
}
