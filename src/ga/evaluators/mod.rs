mod simple_evalutator;


use crate::model::ProblemDomain;

use super::chromosomes::Chromosome;

pub trait FitnessEvaluator {
    type Chromosome: Chromosome;

    fn calculate_fitness(&self, chromosome: &Self::Chromosome, domain: &ProblemDomain) -> f64;
}
