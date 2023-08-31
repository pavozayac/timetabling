use rand::Rng;

use super::Selection;

pub struct StochasticUniversalSampling<R: Rng> {
    rng: R,
}

impl<R: Rng> Selection for StochasticUniversalSampling<R> {
    fn selection<T: super::Chromosome>(&mut self, pool: &[T]) -> Vec<T> {
        todo!()
    }
}
