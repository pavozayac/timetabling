use rand::{thread_rng, Rng};

use crate::model::{
    events::{EventInstance, Schedule},
    resources::{Resource, ResourceIDPair},
    slots::{Outline, Slot},
    EventID, ProblemDomain, SlotID,
};

use crate::utils::is_subset;

pub mod simple_chromosome;

pub trait Chromosome: Clone + Sized {
    fn new<T: IntoIterator<Item = EventInstance>>(event_instances: T) -> Self;
    fn events_count(&self) -> usize;
    fn get_slot(&self, event: EventID) -> SlotID;
    fn set_slot(&mut self, event: EventID, slot: SlotID);
    fn get_resources(&self, event: EventID) -> &[ResourceIDPair];
    fn get_resources_mut(&mut self, event: EventID) -> &mut Vec<ResourceIDPair>;
    fn schedule(&self) -> Result<Schedule, &'static str>;

    fn is_correct(&self, domain: &ProblemDomain) -> bool {
        let mut events_in_bounds = true;
        let mut resources_in_bounds = true;

        for event_id in 0..domain.events.len() {
            if !domain.events[event_id].time_constraints.slots.is_empty()
                && !domain.events[event_id]
                    .time_constraints
                    .slots
                    .contains(&self.get_slot(EventID(event_id)))
            {
                events_in_bounds = false;
            }

            // if !is_subset(
            //     domain.events[event_id]
            //         .resource_constraints
            //         .iter()
            //         .map(|x| (x.0, x.1)),
            //     self.get_resources(EventID(event_id))
            //         .iter()
            //         .map(|x| (x.0, x.1)),
            // ) {
            //     resources_in_bounds = false;
            // }

            for (r_id, r_type_id) in self.get_resources(EventID(event_id)).iter() {
                let r = Resource::new(*r_id, *r_type_id, Outline::new());
                if let Ok(result) = &domain.resources.binary_search(&r) {
                    if !&domain.resources[*result]
                        .availability
                        .slots
                        .contains(&self.get_slot(EventID(event_id)))
                    {
                        resources_in_bounds = false;
                    }
                }
            }

            for req in &domain.events[event_id].resource_requirements {
                if self
                    .get_resources(EventID(event_id))
                    .iter()
                    .fold(0, |acc, x| {
                        if x.1 == req.resource_type_id {
                            acc + 1
                        } else {
                            acc
                        }
                    })
                    < req.amount
                {
                    resources_in_bounds = false;
                }
            }
        }

        events_in_bounds && resources_in_bounds
    }

    fn random(&self, domain: &ProblemDomain) -> Self {
        let mut event_instances: Vec<EventInstance> = vec![];

        let mut rng = thread_rng();

        for e in domain.events.iter() {
            let slot: Slot = SlotID(rng.gen_range(0..domain.max_slots));

            let mut allocated_resources: Vec<ResourceIDPair> = vec![];

            for _ in 0..e
                .resource_requirements
                .iter()
                .fold(0, |acc, x| acc + x.amount)
            {
                allocated_resources.push(
                    domain.resources[rng.gen_range(0..domain.resources.len())]
                        .clone()
                        .into(),
                );
            }

            event_instances.push(EventInstance {
                event_id: e.id,
                slot_id: slot,
                resources: allocated_resources,
            });
        }

        Self::new(event_instances)
    }
}
