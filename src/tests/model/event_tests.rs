use crate::model::{
    events::{EventBuilder, EventInstance, ResourceRequirement},
    resources::Resource,
    slots::{Outline, Slot},
    EventID, ResourceID, ResourceTypeID,
};

#[test]
pub fn assign_errors_with_incorrect_resource_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let event = EventBuilder::new(EventID(1))
        .resource_constraints(vec![Resource::new(
            ResourceID(1),
            ResourceTypeID(1),
            outline.clone(),
        )])
        .build();

    let res = event.assign(
        Slot::new(1),
        vec![Resource::new(ResourceID(2), ResourceTypeID(1), outline)],
    );

    assert_eq!(res, Err(()));
}

#[test]
pub fn assign_succeeds_with_correct_resource_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let event = EventBuilder::new(EventID(1))
        .resource_constraints(vec![Resource::new(
            ResourceID(1),
            ResourceTypeID(1),
            outline.clone(),
        )])
        .build();

    let res = event.clone().assign(
        Slot::new(1),
        vec![Resource::new(
            ResourceID(1),
            ResourceTypeID(1),
            outline.clone(),
        )],
    );

    assert_eq!(
        res,
        Ok(EventInstance {
            event: event,
            assigned_slot: Slot::new(1),
            assigned_resources: vec![Resource::new(ResourceID(1), ResourceTypeID(1), outline)],
        })
    );
}

#[test]
pub fn assign_succeeds_with_no_resource_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let event = EventBuilder::new(EventID(1)).build();

    let res = event.clone().assign(
        Slot::new(1),
        vec![Resource::new(
            ResourceID(1),
            ResourceTypeID(1),
            outline.clone(),
        )],
    );

    assert_eq!(
        res,
        Ok(EventInstance {
            event: event,
            assigned_slot: Slot::new(1),
            assigned_resources: vec![Resource::new(ResourceID(1), ResourceTypeID(1), outline)],
        })
    );
}

#[test]
pub fn assign_succeeds_with_correct_fixed_slot() {
    let event = EventBuilder::new(EventID(1))
        .fixed_slot(Slot::new(1))
        .build();

    let res = event.clone().assign(Slot::new(1), vec![]);

    assert!(matches!(res, Ok(_)));
}

#[test]
pub fn assign_errors_with_wrong_fixed_slot() {
    let event = EventBuilder::new(EventID(1))
        .fixed_slot(Slot::new(2))
        .build();

    let res = event.clone().assign(Slot::new(1), vec![]);

    assert!(matches!(res, Err(_)));
}

#[test]
pub fn assign_succeeds_within_time_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let event = EventBuilder::new(EventID(1))
        .time_constraints(outline)
        .build();

    let res = event.clone().assign(Slot::new(1), vec![]);

    assert!(matches!(res, Ok(_)));
}

#[test]
pub fn assign_errors_outside_time_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(2)]).unwrap();

    let event = EventBuilder::new(EventID(1))
        .time_constraints(outline)
        .build();

    let res = event.clone().assign(Slot::new(1), vec![]);

    assert!(matches!(res, Err(_)));
}

#[test]
pub fn assign_succeeds_with_resource_requirements_fulfilled() {
    let event = EventBuilder::new(EventID(1))
        .resource_requirements(vec![ResourceRequirement::new(ResourceTypeID(1), 1)])
        .build();

    let res = event.clone().assign(
        Slot::new(1),
        vec![Resource::new(
            ResourceID(1),
            ResourceTypeID(1),
            Outline::new(),
        )],
    );

    assert!(matches!(res, Ok(_)));
}

#[test]
pub fn assign_errors_with_resource_requirements_not_fulfilled() {
    let event = EventBuilder::new(EventID(1))
        .resource_requirements(vec![ResourceRequirement::new(ResourceTypeID(1), 1)])
        .build();

    let res = event.clone().assign(
        Slot::new(1),
        vec![Resource::new(
            ResourceID(1),
            ResourceTypeID(2),
            Outline::new(),
        )],
    );

    assert!(matches!(res, Err(_)));
}
