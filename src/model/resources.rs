use std::hash::Hash;

use super::slots::Outline;

#[derive(Clone, Debug)]
pub struct Resource {
    pub id: u64,
    pub type_id: u64,
    pub availability: Outline,
}

impl Resource {
    pub fn new(id: u64, type_id: u64, availability: Outline) -> Resource {
        Resource {
            id,
            type_id,
            availability,
        }
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.type_id == other.type_id;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.id != other.id || self.type_id != other.type_id;
    }
}

impl Eq for Resource {}

impl Hash for Resource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.type_id.hash(state);
    }
}
