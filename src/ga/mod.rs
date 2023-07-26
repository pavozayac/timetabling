pub mod simple_chromosome;

use rand::{thread_rng, Rng};

use crate::model::{
    events::{Event, EventInstance, Schedule},
    resources::Resource,
    slots::{Outline, Slot},
};

pub trait Chromosome: Sized {
    fn new(event_instances: &[EventInstance]) -> Self;
    fn get_slot(&self, event: Event) -> &Slot;
    fn get_slot_mut(&mut self, event: Event) -> &mut Slot;
    fn get_resources(&self, event: Event) -> &[Resource];
    fn get_resources_mut(&mut self, event: Event) -> &mut Vec<Resource>;

    fn is_correct(&self, events: &[Event], outline: Outline, resources: &[Resource]) -> bool;
    fn schedule(&self) -> Result<Schedule, ()>;

    fn random(&self, events: &[Event], outline: Outline, resources: &[Resource]) -> Self {
        let mut event_instances: Vec<EventInstance> = vec![];

        let mut rng = thread_rng();

        for e in events {
            let slot: Slot = outline.slots[rng.gen_range(0..outline.slots.len())];
            let mut allocated_resources: Vec<Resource> = vec![];

            for _ in 0..e
                .resource_requirements
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .fold(0, |acc, x| acc + x.amount)
            {
                allocated_resources.push(resources[rng.gen_range(0..resources.len())].clone());
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
    fn calculate_fitness(
        &self,
        chromosome: T,
        events: &[Event],
        outline: Outline,
        resources: &[Resource],
    ) -> i64;
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
