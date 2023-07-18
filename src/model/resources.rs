use chrono::{DateTime, Utc};

use super::slots::Outline;

#[derive(Clone)]
pub struct Resource {
    pub id: u64,
    pub availability: Outline,
}

impl Resource {
    pub fn new(id: u64, availability: Outline) -> Resource {
        Resource { id, availability }
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.id != other.id;
    }
}
