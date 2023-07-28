use crate::utils::has_unique_items;

use super::SlotID;

pub type Slot = SlotID;

impl Slot {
    pub fn new(id: usize) -> Self {
        SlotID(id)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Outline {
    pub slots: Vec<Slot>,
}

impl Outline {
    pub fn new() -> Self {
        Outline { slots: vec![] }
    }

    pub fn add_slot(&mut self, slot: Slot) -> Result<(), ()> {
        if has_unique_items(self.slots.iter().chain(&[slot])) {
            self.slots.push(slot);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn extend_from_slice(&mut self, slots: &[Slot]) -> Result<(), ()> {
        if has_unique_items(self.slots.iter().chain(slots)) {
            self.slots.extend_from_slice(slots);
            Ok(())
        } else {
            Err(())
        }
    }
}
