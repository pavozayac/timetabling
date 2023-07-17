use chrono::{DateTime, Utc};

pub struct Slot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Slot {
    fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Slot {
        Slot { start, end }
    }

    fn length(&self) -> chrono::Duration {
        self.end.signed_duration_since(self.start)
    }
}

pub struct Outline {
    pub slots: Vec<Slot>,
}

impl Outline {
    fn new() -> Outline {
        Outline { slots: vec![] }
    }

    fn add_slot(&mut self, slot: Slot) {
        self.slots.push(slot);
    }

    fn slots(&self) -> &Vec<Slot> {
        &self.slots
    }
}

impl From<Vec<Slot>> for Outline {
    fn from(value: Vec<Slot>) -> Self {
        Outline { slots: value }
    }
}
