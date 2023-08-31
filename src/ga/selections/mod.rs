mod sus;
pub use sus::*;

mod truncation;
pub use truncation::*;

use super::{chromosomes::Chromosome, evaluators::FitnessEvaluator};

pub trait Selection {
    fn selection<T: Chromosome, F: FitnessEvaluator<Chromosome = T>>(
        &mut self,
        evaluator: F,
        pool: &[T],
    ) -> Vec<T>;
}
