use self::{events::Event, resources::Resource, slots::Outline};

pub mod events;
pub mod resources;
pub mod slots;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventID(pub usize);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotID(pub usize);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceID(pub usize);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceTypeID(pub usize);

impl From<EventID> for usize {
    fn from(value: EventID) -> Self {
        value.0
    }
}

impl From<SlotID> for usize {
    fn from(value: SlotID) -> Self {
        value.0
    }
}

impl From<ResourceID> for usize {
    fn from(value: ResourceID) -> Self {
        value.0
    }
}

impl From<ResourceTypeID> for usize {
    fn from(value: ResourceTypeID) -> Self {
        value.0
    }
}

pub struct ProblemDomain {
    pub events: Vec<Event>,
    pub resources: Vec<Resource>,
    pub max_slots: usize,
}

impl ProblemDomain {
    fn new(events: Vec<Event>, slots_count: usize, resources: Vec<Resource>) -> Self {
        ProblemDomain {
            events,
            resources,
            max_slots: slots_count,
        }
    }
}
