use std::{collections::HashMap, ops::Deref};

use crate::utils::{has_unique_items};

use super::{
    resources::{ResourceIDPair},
    slots::{Outline, Slot},
    EventID, ResourceTypeID, SlotID,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResourceRequirement {
    pub resource_type_id: ResourceTypeID,
    pub amount: u64,
}

impl ResourceRequirement {
    pub fn new(resource_type_id: ResourceTypeID, amount: u64) -> Self {
        ResourceRequirement {
            resource_type_id,
            amount,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    pub id: EventID,
    // pub resource_constraints: Vec<ResourceIDPair>,
    // Some if constrained to only some slots (which should be a subset of the global outline).
    pub time_constraints: Outline,
    pub resource_requirements: Vec<ResourceRequirement>,
}

impl Event {
    pub fn assign(
        self,
        assigned_slot: Slot,
        assigned_resources: Vec<ResourceIDPair>,
    ) -> Result<EventInstance, &'static str> {
        // if let Some(fixed_slot) = self.fixed_slot {
        //     if fixed_slot != assigned_slot {
        //         return Err(());
        //     }
        // }

        if !self.time_constraints.slots.is_empty()
            && !self.time_constraints.slots.contains(&assigned_slot)
        {
            return Err("Slot is not withing time constraints.");
        }

        for rr in self.resource_requirements {
            if assigned_resources.iter().fold(0, |acc, x| {
                if x.1 == rr.resource_type_id {
                    acc + 1
                } else {
                    acc
                }
            }) < rr.amount
            {
                return Err("Resource requirements have not been fulfilled.");
            }
        }

        // let mut temp: Vec<ResourceIDPair> = Vec::new();

        // if utils::is_subset(
        //     if !self.resource_constraints.is_empty() {
        //         self.resource_constraints
        //             .iter()
        //             .for_each(|r| temp.push((r.0, r.1)));
        //         temp.iter()
        //     } else {
        //         assigned_resources.iter()
        //     },
        //     assigned_resources.iter(),
        // ) {
        Ok(EventInstance {
            event_id: self.id,
            slot_id: assigned_slot,
            resources: assigned_resources,
        })
        // } else {
        //     Err(())
        // }
    }
}

pub struct EventBuilder {
    id: EventID,
    resource_constraints: Vec<ResourceIDPair>,
    time_constraints: Outline,
    resource_requirements: Vec<ResourceRequirement>,
}

impl EventBuilder {
    pub fn new(id: EventID) -> EventBuilder {
        EventBuilder {
            id,
            resource_constraints: Vec::new(),
            time_constraints: Outline::new(),
            resource_requirements: Vec::new(),
        }
    }

    // pub fn resource_constraints(mut self, constraints: Vec<ResourceIDPair>) -> Self {
    //     self.resource_constraints = constraints;
    //     self
    // }

    pub fn time_constraints(mut self, constraints: Outline) -> Self {
        self.time_constraints = constraints;
        self
    }

    pub fn resource_requirements(mut self, requirements: Vec<ResourceRequirement>) -> Self {
        self.resource_requirements = requirements;
        self
    }

    pub fn build(self) -> Event {
        Event {
            id: self.id,
            // resource_constraints: self.resource_constraints,
            time_constraints: self.time_constraints,
            resource_requirements: self.resource_requirements,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventInstance {
    pub event_id: EventID,
    pub slot_id: SlotID,
    pub resources: Vec<ResourceIDPair>,
}

impl EventInstance {
    pub fn new(event_id: EventID, slot_id: SlotID, resources: Vec<ResourceIDPair>) -> Self {
        EventInstance {
            event_id,
            slot_id,
            resources,
        }
    }
}

pub struct Schedule {
    pub event_instances: Vec<EventInstance>,
}

impl Schedule {
    pub fn new(event_instances: Vec<EventInstance>) -> Result<Schedule, &'static str> {
        let mut map: HashMap<Slot, Vec<ResourceIDPair>> = HashMap::new();

        // This check ensures that no two EventInstances use the same resources in the same slot
        for ei in event_instances.iter() {
            let key: &Slot = &ei.deref().slot_id;

            if let Some(value) = map.get_mut(key) {
                value.extend_from_slice(ei.resources.as_slice());
            } else {
                let mut initial: Vec<ResourceIDPair> = Vec::new();

                initial.extend_from_slice(ei.resources.as_slice());

                map.insert(*key, initial);
            }
        }

        if map.values().all(|v| has_unique_items(v.iter())) {
            Ok(Schedule { event_instances })
        } else {
            Err("EventInstances occupy the same slot.")
        }
    }
}
