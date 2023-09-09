use rand::{rngs::ThreadRng, Rng};

use super::{Crossover, OnePointCrossover};

pub struct TwoPointCrossover<R: Rng> {
    internal_one_point: OnePointCrossover<R>,
}

impl<R: Rng> TwoPointCrossover<R> {
    pub fn new(rng: R) -> Self {
        Self {
            internal_one_point: OnePointCrossover::new(rng),
        }
    }
}

impl TwoPointCrossover<ThreadRng> {
    pub fn with_thread_rng() -> Self {
        Self {
            internal_one_point: OnePointCrossover::with_thread_rng(),
        }
    }
}

impl<R: Rng> Crossover for TwoPointCrossover<R> {
    fn crossover<T: crate::ga::chromosomes::Chromosome>(&mut self, lhs: T, rhs: T) -> (T, T) {
        let pair: (T, T) = self.internal_one_point.crossover(lhs, rhs);

        self.internal_one_point.crossover(pair.0, pair.1)
    }
}
