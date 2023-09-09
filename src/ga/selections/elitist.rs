use crate::model::ProblemDomain;

use super::Selection;

pub struct ElitistSelection<S: Selection> {
    internal_selection: S,
    pub elite_count: u64,
    pub domain: ProblemDomain,
}

impl<S: Selection> ElitistSelection<S> {
    pub fn new(internal_selection: S, elite_count: u64, domain: ProblemDomain) -> Self {
        Self {
            internal_selection,
            elite_count,
            domain,
        }
    }
}

impl<S: Selection> Selection for ElitistSelection<S> {
    fn selection<
        T: crate::ga::chromosomes::Chromosome,
        F: crate::ga::evaluators::FitnessEvaluator<Chromosome = T>,
    >(
        &mut self,
        evaluator: F,
        pool: &[T],
    ) -> Vec<T> {
        let mut poolvec: Vec<T> = pool.to_vec();

        poolvec.sort_unstable_by(|a, b| {
            evaluator
                .calculate_fitness(b, &self.domain)
                .total_cmp(&evaluator.calculate_fitness(a, &self.domain))
        });

        if pool.len() <= self.elite_count as usize {
            poolvec
        } else {
            let (elites, rest) = poolvec.split_at(self.elite_count as usize);
            let rest_selection: Vec<T> = self.internal_selection.selection(evaluator, rest);

            let mut combined: Vec<T> = elites.to_vec();
            combined.extend(rest_selection.into_iter());

            combined
        }
    }
}
