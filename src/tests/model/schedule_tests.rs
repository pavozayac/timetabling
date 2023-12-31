use crate::model::{
    events::{EventBuilder, Schedule},
    resources::Resource,
    slots::{Outline, Slot},
};

#[test]
pub fn new_instantiates_with_no_conflicts() {
    let event1 = EventBuilder::new(1).build();
    let event2 = EventBuilder::new(2).build();

    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let ei1 = event1
        .assign(Slot::new(1), vec![Resource::new(1, 1, outline.clone())])
        .unwrap();

    let ei2 = event2
        .assign(Slot::new(1), vec![Resource::new(2, 1, outline)])
        .unwrap();

    assert!(matches!(Schedule::new(vec![ei1, ei2]), Ok(_)));
}

#[test]
pub fn new_fails_with_conflicts() {
    let event1 = EventBuilder::new(1).build();
    let event2 = EventBuilder::new(2).build();

    let mut outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let ei1 = event1
        .assign(Slot::new(1), vec![Resource::new(1, 1, outline.clone())])
        .unwrap();

    let ei2 = event2
        .assign(Slot::new(1), vec![Resource::new(1, 1, outline)])
        .unwrap();

    assert!(matches!(Schedule::new(vec![ei1, ei2]), Err(_)));
}
