mod one_point;
pub use one_point::*;

use super::chromosomes::Chromosome;

pub trait Crossover {
    fn crossover<T: Chromosome>(&mut self, lhs: T, rhs: T) -> (T, T);
}
