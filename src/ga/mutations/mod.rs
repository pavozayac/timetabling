mod random_substitution;
pub use random_substitution::*;

use super::chromosomes::Chromosome;

pub trait Mutation {
    fn apply_mutation<T: Chromosome>(&mut self, chromosome: &mut T);
}
