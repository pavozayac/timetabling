use crate::model::{
    events::{EventBuilder, EventInstance},
    resources::Resource,
    slots::{Outline, Slot},
};

#[test]
pub fn errors_with_incorrect_constraints() {
    let event = EventBuilder::new(1)
        .resource_constraints(vec![Resource::new(1, 1, Outline::from(vec![Slot::new(1)]))])
        .build();

    let res = event.assign(
        Slot::new(1),
        vec![Resource::new(2, 1, Outline::from(vec![Slot::new(2)]))],
    );

    assert_eq!(res, Err(()));
}

#[test]
pub fn assigns_with_correct_constraints() {
    let event = EventBuilder::new(1)
        .resource_constraints(vec![Resource::new(1, 1, Outline::from(vec![Slot::new(1)]))])
        .build();

    let res = event.clone().assign(
        Slot::new(1),
        vec![Resource::new(1, 1, Outline::from(vec![Slot::new(1)]))],
    );

    assert_eq!(
        res,
        Ok(EventInstance {
            event: event,
            assigned_slot: Slot::new(1),
            assigned_resources: vec![Resource::new(1, 1, Outline::from(vec![Slot::new(1)],))],
        })
    );
}

#[test]
pub fn assigns_with_no_constraints() {
    let event = EventBuilder::new(1).build();

    let res = event.clone().assign(
        Slot::new(1),
        vec![Resource::new(1, 1, Outline::from(vec![Slot::new(1)]))],
    );

    assert_eq!(
        res,
        Ok(EventInstance {
            event: event,
            assigned_slot: Slot::new(1),
            assigned_resources: vec![Resource::new(1, 1, Outline::from(vec![Slot::new(1)],))],
        })
    );
}
