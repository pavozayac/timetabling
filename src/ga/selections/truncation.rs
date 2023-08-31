use crate::{
    ga::{chromosomes::Chromosome, evaluators::FitnessEvaluator},
    model::ProblemDomain,
};

use super::Selection;

pub struct TruncationSelection<'a> {
    truncation_size: usize,
    problem_domain: &'a ProblemDomain,
}

impl<'a> TruncationSelection<'a> {
    pub fn new(truncation_size: usize, problem_domain: &'a ProblemDomain) -> Self {
        TruncationSelection {
            truncation_size,
            problem_domain,
        }
    }
}

impl<'a> Selection for TruncationSelection<'a> {
    fn selection<T: Chromosome, E: FitnessEvaluator<Chromosome = T>>(
        &mut self,
        evaluator: E,
        pool: &[T],
    ) -> Vec<T> {
        let mut selection: Vec<E::Chromosome> = Vec::new();

        let mut local_pool = pool.to_vec();

        local_pool.sort_unstable_by(|a, b| {
            evaluator
                .calculate_fitness(a, self.problem_domain)
                .total_cmp(&evaluator.calculate_fitness(b, self.problem_domain))
        });

        selection.extend_from_slice(&local_pool[0..self.truncation_size]);

        selection
    }
}
