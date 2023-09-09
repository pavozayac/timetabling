use crate::model::{
    events::{EventBuilder, EventInstance, ResourceRequirement},
    slots::{Outline, Slot},
    EventID, ResourceID, ResourceTypeID,
};

// #[test]
// pub fn assign_errors_with_incorrect_resource_constraints() {
//     let mut outline = Outline::new();
//     outline.extend_from_slice(&[Slot::new(1)]).unwrap();

//     let event = EventBuilder::new(EventID(1))
//         .resource_constraints(vec![ResourceIDPair::new(1, 1)])
//         .build();

//     let res = event.assign(Slot::new(1), vec![(ResourceID(2), ResourceTypeID(1))]);

//     assert_eq!(res, Err(()));
// }

// #[test]
// pub fn assign_succeeds_with_correct_resource_constraints() {
//     let mut outline = Outline::new();
//     outline.extend_from_slice(&[Slot::new(1)]).unwrap();

//     let event = EventBuilder::new(EventID(1))
//         .resource_constraints(vec![ResourceIDPair::new(1, 1)])
//         .build();

//     let res = event
//         .clone()
//         .assign(Slot::new(1), vec![(ResourceID(1), ResourceTypeID(1))]);

//     assert_eq!(
//         res,
//         Ok(EventInstance {
//             event_id: event.id,
//             slot_id: Slot::new(1),
//             resources: vec![(ResourceID(1), ResourceTypeID(1))],
//         })
//     );
// }

#[test]
pub fn assign_succeeds_with_no_resource_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let event = EventBuilder::new(EventID(1)).build();

    let res = event
        .clone()
        .assign(Slot::new(1), vec![(ResourceID(1), ResourceTypeID(1))]);

    assert_eq!(
        res,
        Ok(EventInstance {
            event_id: event.id,
            slot_id: Slot::new(1),
            resources: vec![(ResourceID(1), ResourceTypeID(1))],
        })
    );
}

#[test]
pub fn assign_succeeds_within_time_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let event = EventBuilder::new(EventID(1))
        .time_constraints(outline)
        .build();

    let res = event.assign(Slot::new(1), vec![]);

    assert!(matches!(res, Ok(_)));
}

#[test]
pub fn assign_errors_outside_time_constraints() {
    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(2)]).unwrap();

    let event = EventBuilder::new(EventID(1))
        .time_constraints(outline)
        .build();

    let res = event.assign(Slot::new(1), vec![]);

    assert!(matches!(res, Err(_)));
}

#[test]
pub fn assign_succeeds_with_resource_requirements_fulfilled() {
    let event = EventBuilder::new(EventID(1))
        .resource_requirements(vec![ResourceRequirement::new(ResourceTypeID(1), 1)])
        .build();

    let res = event.assign(Slot::new(1), vec![(ResourceID(1), ResourceTypeID(1))]);

    assert!(matches!(res, Ok(_)));
}

#[test]
pub fn assign_errors_with_resource_requirements_not_fulfilled() {
    let event = EventBuilder::new(EventID(1))
        .resource_requirements(vec![ResourceRequirement::new(ResourceTypeID(1), 1)])
        .build();

    let res = event.assign(Slot::new(1), vec![(ResourceID(1), ResourceTypeID(2))]);

    assert!(matches!(res, Err(_)));
}
