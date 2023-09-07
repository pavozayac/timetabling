use std::hash::Hash;

use super::{slots::Outline, ResourceID, ResourceTypeID};

pub type ResourceIDPair = (ResourceID, ResourceTypeID);

impl From<Resource> for ResourceIDPair {
    fn from(value: Resource) -> Self {
        (value.id, value.type_id)
    }
}

impl From<&Resource> for ResourceIDPair {
    fn from(value: &Resource) -> Self {
        (value.id, value.type_id)
    }
}

pub trait Initializable {
    fn new(id: usize, type_id: usize) -> Self;
}

impl Initializable for ResourceIDPair {
    fn new(id: usize, type_id: usize) -> Self {
        (ResourceID(id), ResourceTypeID(type_id))
    }
}

#[derive(Clone, Debug)]
pub struct Resource {
    pub id: ResourceID,
    pub type_id: ResourceTypeID,
    pub availability: Outline,
}

impl Resource {
    pub fn new(id: ResourceID, type_id: ResourceTypeID, availability: Outline) -> Resource {
        Resource {
            id,
            type_id,
            availability,
        }
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.type_id == other.type_id
    }
}

impl Eq for Resource {}

impl Hash for Resource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.type_id.hash(state);
    }
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.id != other.id {
            self.type_id.partial_cmp(&other.type_id)
        } else {
            self.id.partial_cmp(&other.id)
        }
    }
}

impl Ord for Resource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.id != other.id {
            self.type_id.cmp(&other.type_id)
        } else {
            self.id.cmp(&other.id)
        }
    }
}
