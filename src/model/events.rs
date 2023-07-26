use std::{collections::HashMap, ops::Deref};

use crate::utils::{self, has_unique_items};

use super::{
    resources::Resource,
    slots::{Outline, Slot},
    EventID,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResourceRequirement {
    pub resource_type_id: u64,
    pub amount: u64,
}

impl ResourceRequirement {
    pub fn new(resource_type_id: u64, amount: u64) -> Self {
        ResourceRequirement {
            resource_type_id,
            amount,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    pub id: EventID,
    // This is Some(slot) if the event is always supposed to be in one particular slot.
    pub fixed_slot: Option<Slot>,
    // This is Some if there are any constraints on resources, None if they can be
    // assigned freely.
    pub resource_constraints: Option<Vec<Resource>>,
    // Some if constrained to only some slots (which should be a subset of the global outline).
    pub time_constraints: Option<Outline>,
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
}

impl Event {
    pub fn assign(
        self,
        assigned_slot: Slot,
        assigned_resources: Vec<Resource>,
    ) -> Result<EventInstance, ()> {
        if let Some(fixed_slot) = self.fixed_slot {
            if fixed_slot != assigned_slot {
                return Err(());
            }
        }

        if let Some(constraints) = self.time_constraints.as_ref() {
            if !constraints.slots.contains(&assigned_slot) {
                return Err(());
            }
        }

        if let Some(requirements) = self.resource_requirements.as_ref() {
            for rr in requirements {
                if assigned_resources.iter().fold(0, |acc, x| {
                    if x.type_id == rr.resource_type_id {
                        acc + 1
                    } else {
                        acc
                    }
                }) < rr.amount
                {
                    return Err(());
                }
            }
        }

        if utils::is_subset(
            self.resource_constraints
                .as_ref()
                .unwrap_or(&assigned_resources),
            &assigned_resources,
        ) {
            Ok(EventInstance {
                event: self,
                assigned_slot: assigned_slot,
                assigned_resources: assigned_resources,
            })
        } else {
            return Err(());
        }
    }
}

pub struct EventBuilder {
    id: EventID,
    fixed_slot: Option<Slot>,
    resource_constraints: Option<Vec<Resource>>,
    time_constraints: Option<Outline>,
    resource_requirements: Option<Vec<ResourceRequirement>>,
}

impl EventBuilder {
    pub fn new(id: EventID) -> EventBuilder {
        EventBuilder {
            id: id,
            fixed_slot: None,
            resource_constraints: None,
            time_constraints: None,
            resource_requirements: None,
        }
    }

    pub fn fixed_slot(mut self, slot: Slot) -> Self {
        self.fixed_slot = Some(slot);
        self
    }

    pub fn resource_constraints(mut self, constraints: Vec<Resource>) -> Self {
        self.resource_constraints = Some(constraints);
        self
    }

    pub fn time_constraints(mut self, constraints: Outline) -> Self {
        self.time_constraints = Some(constraints);
        self
    }

    pub fn resource_requirements(mut self, requirements: Vec<ResourceRequirement>) -> Self {
        self.resource_requirements = Some(requirements);
        self
    }

    pub fn build(self) -> Event {
        Event {
            id: self.id,
            fixed_slot: self.fixed_slot,
            resource_constraints: self.resource_constraints,
            time_constraints: self.time_constraints,
            resource_requirements: self.resource_requirements,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventInstance {
    pub event: Event,
    pub assigned_slot: Slot,
    pub assigned_resources: Vec<Resource>,
}

pub struct Schedule {
    pub event_instances: Vec<EventInstance>,
}

impl Schedule {
    pub fn new(event_instances: Vec<EventInstance>) -> Result<Schedule, ()> {
        let mut map: HashMap<Slot, Vec<Resource>> = HashMap::new();

        // This check ensures that no two EventInstances use the same resources in the same slot
        for ei in event_instances.iter() {
            let key: &Slot = &ei.deref().assigned_slot;

            if let Some(value) = map.get_mut(key) {
                value.extend_from_slice(ei.assigned_resources.as_slice());
            } else {
                let mut initial: Vec<Resource> = Vec::new();

                initial.extend_from_slice(ei.assigned_resources.as_slice());

                map.insert(*key, initial);
            }
        }

        if map.values().all(|v| has_unique_items(v.iter())) {
            Ok(Schedule { event_instances })
        } else {
            Err(())
        }
    }
}
