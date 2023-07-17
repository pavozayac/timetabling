use chrono::{DateTime, Utc};

use super::events::EventInstance;

pub struct Slot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Slot {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Slot {
        Slot { start, end }
    }

    pub fn length(&self) -> chrono::Duration {
        self.end.signed_duration_since(self.start)
    }

    pub fn populate(self, event_instances: Vec<EventInstance>) -> PopulatedSlot {
        PopulatedSlot {
            slot: self,
            event_instances: event_instances,
        }
    }
}

pub struct Outline {
    pub slots: Vec<Slot>,
}

impl Outline {
    pub fn new() -> Outline {
        Outline { slots: vec![] }
    }

    pub fn add_slot(&mut self, slot: Slot) {
        self.slots.push(slot);
    }

    pub fn slots(&self) -> &Vec<Slot> {
        &self.slots
    }
}

impl From<Vec<Slot>> for Outline {
    fn from(value: Vec<Slot>) -> Self {
        Outline { slots: value }
    }
}

pub struct PopulatedSlot {
    pub slot: Slot,
    pub event_instances: Vec<EventInstance>,
}
