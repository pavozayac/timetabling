use crate::{
    model::{
        events::{EventBuilder, EventInstance, Schedule},
        resources::Resource,
        slots::{Outline, Slot},
        EventID, ProblemDomain, ResourceID, ResourceTypeID, SlotID,
    },
    utils::is_subset,
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
            slot_allocations: slot_allocs.iter().map(|x| x.assigned_slot.id).collect(),
            resource_allocations: resource_allocs,
        }
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

    fn get_resources_mut(&mut self, event: EventID) -> &mut Vec<(ResourceID, ResourceTypeID)> {
        &mut self.resource_allocations[usize::from(event)]
    }

    fn is_correct(&self, domain: &ProblemDomain) -> bool {
        let mut events_in_bounds = true;
        let mut resources_in_bounds = true;

        for event_id in 0..self.slot_allocations.len() {
            if let Some(constraints) = &domain.events[event_id].time_constraints {
                if !constraints
                    .slots
                    .contains(&Slot::new(self.get_slot(EventID(event_id))))
                {
                    events_in_bounds = false;
                }
            }

            if let Some(requirements) = &domain.events[event_id].resource_constraints {
                if !is_subset(
                    requirements.iter().map(|x| (x.id, x.type_id)),
                    self.resource_allocations[event_id]
                        .iter()
                        .map(|x| (x.0, x.1)),
                ) {
                    resources_in_bounds = false;
                }
            }

            for (r_id, r_type_id) in self.resource_allocations[event_id].iter() {
                let r = Resource::new(*r_id, *r_type_id, Outline::new());
                if let Ok(result) = &domain.resources.binary_search(&r) {
                    if !&domain.resources[*result]
                        .availability
                        .slots
                        .contains(&Slot::new(self.slot_allocations[event_id]))
                    {
                        resources_in_bounds = false;
                    }
                }
            }
        }

        self.schedule().is_ok() && events_in_bounds && resources_in_bounds
    }

    fn schedule(&self) -> Result<Schedule, ()> {
        let mut event_instances = vec![];

        for (i, e) in self.slot_allocations.iter().enumerate() {
            let instance = EventBuilder::new(EventID(i)).build().assign(
                Slot::new(*e),
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
