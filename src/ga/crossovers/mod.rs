use rand::{rngs::ThreadRng, Rng};

use crate::model::{events::EventInstance, EventID};

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

        let mut low_left: Vec<EventInstance> = Vec::new();
        let mut high_left: Vec<EventInstance> = Vec::new();
        let mut low_right: Vec<EventInstance> = Vec::new();
        let mut high_right: Vec<EventInstance> = Vec::new();

        for i in 0..lcount {
            if i < cross_point {
                low_left.push(EventInstance::new(
                    EventID(i),
                    lhs.get_slot(EventID(i)),
                    lhs.get_resources(EventID(i))
                        .iter()
                        .map(|r| (*r).into())
                        .collect(),
                ));

                low_right.push(EventInstance::new(
                    EventID(i),
                    rhs.get_slot(EventID(i)),
                    rhs.get_resources(EventID(i))
                        .iter()
                        .map(|r| (*r).into())
                        .collect(),
                ));
            } else {
                high_left.push(EventInstance::new(
                    EventID(i),
                    lhs.get_slot(EventID(i)),
                    lhs.get_resources(EventID(i))
                        .iter()
                        .map(|r| (*r).into())
                        .collect(),
                ));

                high_right.push(EventInstance::new(
                    EventID(i),
                    rhs.get_slot(EventID(i)),
                    rhs.get_resources(EventID(i))
                        .iter()
                        .map(|r| (*r).into())
                        .collect(),
                ));
            }
        }

        let child1: T = T::new(low_left.into_iter().chain(high_right.into_iter()));
        let child2: T = T::new(low_right.into_iter().chain(high_left.into_iter()));

        (child1, child2)
    }
}