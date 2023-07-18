use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::model::{
    events::{Event, EventBuilder, EventInstance},
    resources::Resource,
    slots::{Outline, Slot},
};

#[test]
pub fn event_errors_with_wrong_constraints() {
    let event = EventBuilder::new(1, Duration::minutes(30))
        .resource_constraints(vec![Resource::new(1, Outline::from(vec![Slot::new(1)]))])
        .build();

    let res = event.assign(vec![Resource::new(2, Outline::from(vec![Slot::new(2)]))]);

    assert_eq!(res, Err(()));
}

#[test]
pub fn event_assigns_with_constraints() {
    let event = EventBuilder::new(1, Duration::minutes(30))
        .resource_constraints(vec![Resource::new(1, Outline::from(vec![Slot::new(1)]))])
        .build();

    let res = event
        .clone()
        .assign(vec![Resource::new(1, Outline::from(vec![Slot::new(1)]))]);

    assert_eq!(
        res,
        Ok(EventInstance {
            event: event,
            assigned_resources: vec![Resource::new(1, Outline::from(vec![Slot::new(1)],))],
        })
    );
}
