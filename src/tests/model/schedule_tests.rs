use crate::model::{
    events::{EventBuilder, Schedule},
    resources::Resource,
    slots::{Outline, Slot},
    EventID, ResourceID, ResourceTypeID,
};

#[test]
pub fn new_instantiates_with_no_conflicts() {
    let event1 = EventBuilder::new(EventID(1)).build();
    let event2 = EventBuilder::new(EventID(2)).build();

    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let ei1 = event1
        .assign(
            Slot::new(1),
            vec![Resource::new(
                ResourceID(1),
                ResourceTypeID(1),
                outline.clone(),
            )],
        )
        .unwrap();

    let ei2 = event2
        .assign(
            Slot::new(1),
            vec![Resource::new(ResourceID(2), ResourceTypeID(1), outline)],
        )
        .unwrap();

    assert!(matches!(Schedule::new(vec![ei1, ei2]), Ok(_)));
}

#[test]
pub fn new_fails_with_conflicts() {
    let event1 = EventBuilder::new(EventID(1)).build();
    let event2 = EventBuilder::new(EventID(2)).build();

    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let ei1 = event1
        .assign(
            Slot::new(1),
            vec![Resource::new(
                ResourceID(1),
                ResourceTypeID(1),
                outline.clone(),
            )],
        )
        .unwrap();

    let ei2 = event2
        .assign(
            Slot::new(1),
            vec![Resource::new(ResourceID(1), ResourceTypeID(1), outline)],
        )
        .unwrap();

    assert!(matches!(Schedule::new(vec![ei1, ei2]), Err(_)));
}
