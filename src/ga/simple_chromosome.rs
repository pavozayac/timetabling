use crate::model::{
    events::{EventBuilder, EventInstance, Schedule},
    resources::Resource,
    slots::Outline,
    EventID, ResourceID, ResourceTypeID, SlotID,
};

use super::Chromosome;

// Implementation assumes events, slots and resources are given as a contiguous group
// starting with id 0
pub struct SimpleChromosome {
    slot_allocations: Vec<SlotID>,
    resource_allocations: Vec<Vec<(ResourceID, ResourceTypeID)>>,
}

impl Chromosome for SimpleChromosome {
    fn new(event_instances: &[EventInstance]) -> Self {
        let mut slot_allocs = event_instances.to_vec();

        slot_allocs.sort_unstable_by(|a, b| a.event.id.cmp(&b.event.id));

        let resource_allocs = event_instances
            .iter()
            .map(|x| {
                x.assigned_resources
                    .iter()
                    .map(|r| (r.id, r.type_id))
                    .collect()
            })
            .collect();

        SimpleChromosome {
            slot_allocations: slot_allocs.iter().map(|x| x.assigned_slot).collect(),
            resource_allocations: resource_allocs,
        }
    }

    fn events_count(&self) -> usize {
        self.slot_allocations.len()
    }

    fn get_slot(&self, event: EventID) -> SlotID {
        self.slot_allocations[usize::from(event)]
    }

    fn set_slot(&mut self, event: EventID, slot: SlotID) {
        self.slot_allocations[usize::from(event)] = slot;
    }

    fn get_resources(&self, event: EventID) -> &[(ResourceID, ResourceTypeID)] {
        &self.resource_allocations[usize::from(event)]
    }

    fn get_resources_mut(&mut self, event: EventID) -> &mut [(ResourceID, ResourceTypeID)] {
        &mut self.resource_allocations[usize::from(event)]
    }

    fn schedule(&self) -> Result<Schedule, ()> {
        let mut event_instances = vec![];

        for (i, e) in self.slot_allocations.iter().enumerate() {
            let instance = EventBuilder::new(EventID(i)).build().assign(
                *e,
                self.resource_allocations[i]
                    .iter()
                    .map(|(r, t)| Resource::new(*r, *t, Outline::new()))
                    .collect(),
            )?;

            event_instances.push(instance);
        }

        Schedule::new(event_instances)
    }
}
