use chrono::{DateTime, Utc};

use super::slots::Outline;

#[derive(Clone, Copy)]
pub struct Resource {
    pub name: String,
    pub availability: Outline,
}

impl Resource {
    pub fn new(name: String, availability: Outline) -> Resource {
        Resource { name, availability }
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.name != other.name;
    }
}
