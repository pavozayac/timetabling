use crate::utils::has_unique_items;

use super::events::EventInstance;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Slot {
    id: i64,
}

impl Slot {
    pub fn new(id: i64) -> Slot {
        Slot { id }
    }

    pub fn populate(self, event_instances: Vec<EventInstance>) -> Result<PopulatedSlot, ()> {
        let folded = event_instances.iter().fold(vec![], |mut acc, x| {
            acc.extend_from_slice(x.assigned_resources.as_slice());
            acc
        });

        if !has_unique_items(folded) {
            return Err(());
        }

        Ok(PopulatedSlot {
            slot: self,
            event_instances: event_instances,
        })
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

#[derive(Clone)]
pub struct PopulatedSlot {
    pub slot: Slot,
    pub event_instances: Vec<EventInstance>,
}

pub struct Schedule {
    pub populated_slots: Vec<PopulatedSlot>,
}

impl Schedule {
    pub fn new(populated_slots: Vec<PopulatedSlot>) -> Schedule {
        Schedule { populated_slots }
    }
}
