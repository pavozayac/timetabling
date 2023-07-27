pub mod simple_chromosome;

use rand::{thread_rng, Rng};

use crate::model::{
    events::{Event, EventInstance, Schedule},
    resources::Resource,
    slots::{Outline, Slot},
    EventID, ProblemDomain, ResourceID, ResourceTypeID, SlotID,
};

pub trait Chromosome: Sized {
    fn new(event_instances: &[EventInstance]) -> Self;
    fn get_slot(&self, event: EventID) -> SlotID;
    fn set_slot(&mut self, event: EventID, slot: SlotID);
    fn get_resources(&self, event: EventID) -> &[(ResourceID, ResourceTypeID)];
    fn get_resources_mut(&mut self, event: EventID) -> &mut Vec<(ResourceID, ResourceTypeID)>;

    fn is_correct(&self, domain: &ProblemDomain) -> bool;
    fn schedule(&self) -> Result<Schedule, ()>;

    fn random(&self, domain: &ProblemDomain) -> Self {
        let mut event_instances: Vec<EventInstance> = vec![];

        let mut rng = thread_rng();

        for e in domain.events.iter() {
            let slot: Slot = domain.outline.slots[rng.gen_range(0..domain.outline.slots.len())];
            let mut allocated_resources: Vec<Resource> = vec![];

            for _ in 0..e
                .resource_requirements
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .fold(0, |acc, x| acc + x.amount)
            {
                allocated_resources
                    .push(domain.resources[rng.gen_range(0..domain.resources.len())].clone());
            }

            event_instances.push(EventInstance {
                event: e.clone(),
                assigned_slot: slot,
                assigned_resources: allocated_resources,
            });
        }

        Self::new(&event_instances)
    }
}

pub trait FitnessEvaluator<T: Chromosome> {
    fn calculate_fitness(&self, chromosome: T, domain: &ProblemDomain) -> i64;
}

pub trait Mutation {
    fn mutation<T: Chromosome>(&self, chromosome: T) -> T;
}

pub trait Crossover {
    fn crossover<T: Chromosome>(&self, lhs: T, rhs: T) -> T;
}

pub trait Selection {
    fn selection<T: Chromosome>(&self, pool: Vec<T>) -> Vec<T>;
}
