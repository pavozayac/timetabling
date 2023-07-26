pub mod events;
pub mod resources;
pub mod slots;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventID(pub usize);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotID(pub usize);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceID(pub usize);
