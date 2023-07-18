use chrono::Duration;

use crate::utils;

use super::{
    resources::Resource,
    slots::{Outline, Slot},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    pub id: u64,
    pub length: Duration,
    // This is Some(slot) if the event is always supposed to be in one particular slot.
    pub fixed_slot: Option<Slot>,
    // This is Some if there are any constraints on resources, None if they can be
    // assigned freely.
    pub resource_constraints: Option<Vec<Resource>>,
    // Some if constrained to only some slots (which should be a subset of the global outline).
    pub time_constraints: Option<Outline>,
    // This makes it rather easy to specify the repeats (e.g. daily, weekly,...).
    // If None, then event occurs only once.
    pub repeat_duration: Option<Duration>,
    pub tags: Vec<u64>,
}

impl Event {
    pub fn new(
        id: u64,
        length: Duration,
        fixed_slot: Option<Slot>,
        resource_constraints: Option<Vec<Resource>>,
        time_constraints: Option<Outline>,
        repeat_duration: Option<Duration>,
        tags: Vec<u64>,
    ) -> Event {
        Event {
            id,
            length,
            fixed_slot,
            resource_constraints,
            time_constraints,
            repeat_duration,
            tags,
        }
    }

    pub fn assign(self, assigned_resources: Vec<Resource>) -> Result<EventInstance, ()> {
        if utils::is_subset(
            self.resource_constraints.as_ref().unwrap_or(&vec![]),
            &assigned_resources,
        ) {
            return Err(());
        }

        Ok(EventInstance {
            event: self,
            assigned_resources: assigned_resources,
        })
    }
}

pub struct EventBuilder {
    id: u64,
    length: Duration,
    fixed_slot: Option<Slot>,
    resource_constraints: Option<Vec<Resource>>,
    time_constraints: Option<Outline>,
    repeat_duration: Option<Duration>,
    tags: Vec<u64>,
}

impl EventBuilder {
    pub fn new(id: u64, length: Duration) -> EventBuilder {
        EventBuilder {
            id: id,
            length: length,
            fixed_slot: None,
            resource_constraints: None,
            time_constraints: None,
            repeat_duration: None,
            tags: vec![],
        }
    }

    pub fn fixed_slot(mut self, slot: Slot) -> EventBuilder {
        self.fixed_slot = Some(slot);
        self
    }

    pub fn resource_constraints(mut self, constraints: Vec<Resource>) -> EventBuilder {
        self.resource_constraints = Some(constraints);
        self
    }

    pub fn time_constraints(mut self, constraints: Outline) -> EventBuilder {
        self.time_constraints = Some(constraints);
        self
    }

    pub fn repeat_duration(mut self, duration: Duration) -> EventBuilder {
        self.repeat_duration = Some(duration);
        self
    }

    pub fn tags(mut self, tags: Vec<u64>) -> EventBuilder {
        self.tags = tags;
        self
    }

    pub fn build(self) -> Event {
        Event {
            id: self.id,
            length: self.length,
            fixed_slot: self.fixed_slot,
            resource_constraints: self.resource_constraints,
            time_constraints: self.time_constraints,
            repeat_duration: self.repeat_duration,
            tags: self.tags,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventInstance {
    pub event: Event,
    pub assigned_resources: Vec<Resource>,
}
