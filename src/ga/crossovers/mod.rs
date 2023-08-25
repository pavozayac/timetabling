use rand::{rngs::ThreadRng, Rng};

use crate::model::events::EventInstance;

use super::Crossover;

pub struct OnePointCrossover<T: Rng> {
    rng: T,
}

impl<T: Rng> OnePointCrossover<T> {
    fn new(rng: T) -> Self {
        OnePointCrossover { rng }
    }
}

impl OnePointCrossover<ThreadRng> {
    fn with_thread_rng() -> Self {
        OnePointCrossover {
            rng: rand::thread_rng(),
        }
    }
}

impl<R: Rng> Crossover for OnePointCrossover<R> {
    fn crossover<T: super::Chromosome>(&mut self, lhs: T, rhs: T) -> (T, T) {
        let lcount: usize = lhs.events_count();

        assert!(lcount == rhs.events_count());

        let cross_point: usize = self.rng.gen_range(0..lcount);

        let low_left: Vec<EventInstance> = Vec::new();
        let high_left: Vec<EventInstance> = Vec::new();
        let low_right: Vec<EventInstance> = Vec::new();
        let high_right: Vec<EventInstance> = Vec::new();

        // for i in 0..cross_point {
        //     low_left.push(lhs.get_slot(i).)
        // }

        todo!()
    }
}
