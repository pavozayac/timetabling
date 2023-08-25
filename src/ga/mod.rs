pub mod chromosomes;
pub mod crossovers;

use rand::{thread_rng, Rng};

use crate::{
    model::{
        events::{EventInstance, Schedule},
        resources::{Resource, ResourceIDPair},
        slots::{Outline, Slot},
        EventID, ProblemDomain, ResourceID, ResourceTypeID, SlotID,
    },
    utils::is_subset,
};

pub trait Chromosome: Sized {
    fn new(event_instances: &[EventInstance]) -> Self;
    fn events_count(&self) -> usize;
    fn get_slot(&self, event: EventID) -> SlotID;
    fn set_slot(&mut self, event: EventID, slot: SlotID);
    fn get_resources(&self, event: EventID) -> &[(ResourceID, ResourceTypeID)];
    fn get_resources_mut(&mut self, event: EventID) -> &mut [(ResourceID, ResourceTypeID)];
    fn schedule(&self) -> Result<Schedule, ()>;

    fn is_correct(&self, domain: &ProblemDomain) -> bool {
        let mut events_in_bounds = true;
        let mut resources_in_bounds = true;

        for event_id in 0..domain.events.len() {
            if let Some(constraints) = &domain.events[event_id].time_constraints {
                if !constraints
                    .slots
                    .contains(&self.get_slot(EventID(event_id)))
                {
                    events_in_bounds = false;
                }
            }

            if let Some(fixed_slot) = &domain.events[event_id].fixed_slot {
                if self.get_slot(EventID(event_id)) != *fixed_slot {
                    events_in_bounds = false;
                }
            }

            if let Some(requirements) = &domain.events[event_id].resource_constraints {
                if !is_subset(
                    requirements.iter().map(|x| (x.id, x.type_id)),
                    self.get_resources(EventID(event_id))
                        .iter()
                        .map(|x| (x.0, x.1)),
                ) {
                    resources_in_bounds = false;
                }
            }

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

            for reqs in &domain.events[event_id].resource_requirements {
                for req in reqs {
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
        }

        events_in_bounds && resources_in_bounds
    }

    fn random(&self, domain: &ProblemDomain) -> Self {
        let mut event_instances: Vec<EventInstance> = vec![];

        let mut rng = thread_rng();

        for e in domain.events.iter() {
            let slot: Slot = domain.outline.slots[rng.gen_range(0..domain.outline.slots.len())];
            let mut allocated_resources: Vec<ResourceIDPair> = vec![];

            for _ in 0..e
                .resource_requirements
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .fold(0, |acc, x| acc + x.amount)
            {
                allocated_resources
                    .push(domain.resources[rng.gen_range(0..domain.resources.len())].into());
            }

            event_instances.push(EventInstance {
                event_id: e.id,
                slot_id: slot,
                resources: allocated_resources,
            });
        }

        Self::new(&event_instances)
    }
}

pub trait FitnessEvaluator<T: Chromosome> {
    fn calculate_fitness(&self, chromosome: T, domain: &ProblemDomain) -> f64;
}

pub trait Mutation {
    fn apply_mutation<T: Chromosome>(&mut self, chromosome: &mut T);
}

pub trait Crossover {
    fn crossover<T: Chromosome>(&mut self, lhs: T, rhs: T) -> (T, T);
}

pub trait Selection {
    fn selection<T: Chromosome>(&mut self, pool: &[T]) -> Vec<T>;
}
