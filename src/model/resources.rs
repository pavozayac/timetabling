use chrono::{DateTime, Utc};

use super::slots::Outline;

pub struct Resource {
    pub name: String,
    pub availability: Outline,
}

impl Resource {
    pub fn new(name: String, availability: Outline) -> Resource {
        Resource { name, availability }
    }
}