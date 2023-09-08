use std::{collections::HashMap};

use crate::{
    ga::chromosomes::{simple_chromosome::SimpleChromosome, Chromosome},
    model::{
        resources::ResourceIDPair, slots::Slot, EventID,
        ResourceTypeID,
    },
};

use super::FitnessEvaluator;

pub struct SimpleEvaluator {}

impl FitnessEvaluator for SimpleEvaluator {
    type Chromosome = SimpleChromosome;

    fn calculate_fitness(
        &self,
        chromosome: &Self::Chromosome,
        domain: &crate::model::ProblemDomain,
    ) -> f64 {
        let mut score = 0.0;

        // The weighting is in progress.

        for i in 0..chromosome.events_count() {
            let resources: &[ResourceIDPair] = chromosome.get_resources(EventID(i));
            let slot: Slot = chromosome.get_slot(EventID(i));

            if domain.events[i].time_constraints.slots.contains(&slot) {
                score += 1.0;
            } else {
                score -= 1.0;
            }

            let mut count_map: HashMap<ResourceTypeID, u64> = HashMap::new();

            for r in resources {
                if let Some(count) = count_map.get_mut(&r.1) {
                    *count += 1;
                } else {
                    count_map.insert(r.1, 1);
                }
            }

            for rr in domain.events[i].resource_requirements.iter() {
                if let Some(actual_amount) = count_map.get(&rr.resource_type_id) {
                    // 1.0 is for all requirements met, linear scaling for partially met.
                    score += *actual_amount.min(&rr.amount) as f64 / rr.amount as f64;
                } else {
                    // Full penalty for no compliance.
                    score -= 1.0;
                }
            }
        }

        score
    }
}
