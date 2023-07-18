use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::model::{
    events::{Event, EventBuilder, EventInstance},
    resources::Resource,
    slots::{Outline, Slot},
};

#[test]
pub fn event_errors_with_wrong_constraints() {
    let event = EventBuilder::new(1, Duration::minutes(30))
        .resource_constraints(vec![Resource::new(
            1,
            Outline::from(vec![Slot::new(
                Utc.with_ymd_and_hms(2023, 7, 18, 17, 55, 39).unwrap(),
                Utc.with_ymd_and_hms(2023, 7, 18, 18, 55, 39).unwrap(),
            )]),
        )])
        .build();

    let res = event.assign(vec![Resource::new(
        2,
        Outline::from(vec![Slot::new(
            Utc.with_ymd_and_hms(2023, 7, 18, 17, 55, 39).unwrap(),
            Utc.with_ymd_and_hms(2023, 7, 18, 18, 55, 39).unwrap(),
        )]),
    )]);

    assert_eq!(res, Err(()));
}

#[test]
pub fn event_assigns_with_constraints() {
    let event = EventBuilder::new(1, Duration::minutes(30))
        .resource_constraints(vec![Resource::new(
            1,
            Outline::from(vec![Slot::new(
                Utc.with_ymd_and_hms(2023, 7, 18, 17, 55, 39).unwrap(),
                Utc.with_ymd_and_hms(2023, 7, 18, 18, 55, 39).unwrap(),
            )]),
        )])
        .build();

    let res = event.clone().assign(vec![Resource::new(
        1,
        Outline::from(vec![Slot::new(
            Utc.with_ymd_and_hms(2023, 7, 18, 17, 55, 39).unwrap(),
            Utc.with_ymd_and_hms(2023, 7, 18, 18, 55, 39).unwrap(),
        )]),
    )]);

    assert_eq!(
        res,
        Ok(EventInstance {
            event: event,
            assigned_resources: vec![Resource::new(
                1,
                Outline::from(vec![Slot::new(
                    Utc.with_ymd_and_hms(2023, 7, 18, 17, 55, 39).unwrap(),
                    Utc.with_ymd_and_hms(2023, 7, 18, 18, 55, 39).unwrap(),
                )],)
            )],
        })
    );
}
