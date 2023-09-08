use rand::Rng;

use crate::ga::{chromosomes::Chromosome, evaluators::FitnessEvaluator};

use super::Selection;

pub struct StochasticUniversalSampling<R: Rng> {
    rng: R,
}

impl<R: Rng> Selection for StochasticUniversalSampling<R> {
    fn selection<T: Chromosome, E: FitnessEvaluator<Chromosome = T>>(
        &mut self,
        _evaluator: E,
        _pool: &[T],
    ) -> Vec<T> {
        todo!()
    }
}
