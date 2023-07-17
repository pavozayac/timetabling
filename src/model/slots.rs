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
