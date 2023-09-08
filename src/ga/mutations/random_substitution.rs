use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::model::{resources::ResourceIDPair, slots::Slot, EventID, ProblemDomain, SlotID};

use super::Mutation;

pub struct RandomSubstitutionMutation<'a, R: Rng> {
    rng: R,
    mutation_rate: f64,
    problem_domain: &'a ProblemDomain,
}

impl<'a, R: Rng> RandomSubstitutionMutation<'a, R> {
    pub fn new(rng: R, mutation_rate: f64, problem_domain: &'a ProblemDomain) -> Self {
        RandomSubstitutionMutation {
            rng,
            mutation_rate,
            problem_domain,
        }
    }

    pub fn with_thread_rng(
        mutation_rate: f64,
        problem_domain: &ProblemDomain,
    ) -> RandomSubstitutionMutation<ThreadRng> {
        RandomSubstitutionMutation {
            rng: thread_rng(),
            mutation_rate,
            problem_domain,
        }
    }
}

impl<'a, R: Rng> Mutation for RandomSubstitutionMutation<'a, R> {
    fn apply_mutation<T: super::Chromosome>(&mut self, chromosome: &mut T) {
        let mut counter: f64 = self.mutation_rate;

        while counter >= 1.0 {
            let slot_substitute: Slot =
                SlotID(self.rng.gen_range(0..self.problem_domain.max_slots));

            let event_choice: EventID =
                EventID(self.rng.gen_range(0..self.problem_domain.events.len()));

            chromosome.set_slot(event_choice, slot_substitute);

            let resources: &mut Vec<ResourceIDPair> = chromosome.get_resources_mut(EventID(
                self.rng.gen_range(0..self.problem_domain.events.len()),
            ));

            // 50% chance to either extend the list of resources assigned to the event,
            // or to substitute for another resource instead.
            let chance = if resources.len() > 0 {
                1.0 / (resources.len() as f64 + 1.0)
            } else {
                0.0
            };

            let delete: bool = self.rng.gen_bool(chance);

            if delete {
                resources.swap_remove(self.rng.gen_range(0..resources.len()));
            } else {
                let resource_substitute: ResourceIDPair = self
                    .problem_domain
                    .resources
                    .choose(&mut self.rng)
                    .unwrap()
                    .into();

                if resources.is_empty() || self.rng.gen_bool(chance) {
                    resources.push(resource_substitute);
                } else {
                    *resources.choose_mut(&mut self.rng).unwrap() = resource_substitute;
                }
            }

            counter -= 1.0;
        }
    }
}
