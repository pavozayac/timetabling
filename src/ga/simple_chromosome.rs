use crate::model::{
    events::{Event, EventBuilder, EventInstance, Schedule},
    resources::Resource,
    slots::{Outline, Slot},
};

use super::Chromosome;

// Implementation assumes events are given as a contiguous group
// starting with id 0
pub struct SimpleChromosome {
    slot_allocations: Vec<u64>,
    resource_allocations: Vec<Vec<(u64, u64)>>,
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
            slot_allocations: slot_allocs.iter().map(|x| x.assigned_slot.id).collect(),
            resource_allocations: resource_allocs,
        }
    }

    fn has_allocation(&self, event: Event, slot: Slot) -> bool {
        if let Some(slot_id) = self.slot_allocations.get(event.id as usize) {
            *slot_id == slot.id
        } else {
            false
        }
    }

    fn has_resource(&self, event: Event, resource: Resource) -> bool {
        if let Some(resources) = self.resource_allocations.get(event.id as usize) {
            resources.contains(&(resource.id, resource.type_id))
        } else {
            false
        }
    }

    fn is_correct(&self, events: &[Event], outline: Outline, resources: &[Resource]) -> bool {
        self.schedule().is_ok()
    }

    fn schedule(&self) -> Result<Schedule, ()> {
        let mut event_instances = vec![];

        for (i, e) in self.slot_allocations.iter().enumerate() {
            let instance = EventBuilder::new(*e).build().assign(
                Slot::new(i as u64),
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
