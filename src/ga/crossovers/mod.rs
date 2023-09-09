mod one_point;
pub use one_point::*;

mod two_point;
pub use two_point::*;

use super::chromosomes::Chromosome;

pub trait Crossover {
    fn crossover<T: Chromosome>(&mut self, lhs: T, rhs: T) -> (T, T);
}
