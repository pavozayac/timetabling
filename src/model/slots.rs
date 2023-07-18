use chrono::{DateTime, Duration, Utc};

use super::events::EventInstance;

#[derive(Clone, Copy)]
pub struct Slot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Slot {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Slot {
        Slot { start, end }
    }

    pub fn length(&self) -> Duration {
        self.end.signed_duration_since(self.start)
    }

    pub fn populate(self, event_instances: Vec<EventInstance>) -> PopulatedSlot {
        let all_assigned = event_instances.iter().fold(vec![], |mut acc, x| {
            acc.extend_from_slice(&x.assigned_resources);
            acc
        });

        // if all_assigned.

        PopulatedSlot {
            slot: self,
            event_instances: event_instances,
        }
    }
}

#[derive(Clone)]
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
