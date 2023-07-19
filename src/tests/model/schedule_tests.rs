use crate::model::{
    events::{EventBuilder, Schedule},
    resources::Resource,
    slots::{Outline, Slot},
};

#[test]
pub fn instantiates_with_no_conflicts() {
    let event1 = EventBuilder::new(1).build();
    let event2 = EventBuilder::new(2).build();

    let ei1 = event1
        .assign(
            Slot::new(1),
            vec![Resource::new(1, Outline::from(vec![Slot::new(1)]))],
        )
        .unwrap();

    let ei2 = event2
        .assign(
            Slot::new(1),
            vec![Resource::new(2, Outline::from(vec![Slot::new(1)]))],
        )
        .unwrap();

    assert!(matches!(Schedule::new(vec![ei1, ei2]), Ok(_)));
}

#[test]
pub fn fails_with_conflicts() {
    let event1 = EventBuilder::new(1).build();
    let event2 = EventBuilder::new(2).build();

    let ei1 = event1
        .assign(
            Slot::new(1),
            vec![Resource::new(1, Outline::from(vec![Slot::new(1)]))],
        )
        .unwrap();

    let ei2 = event2
        .assign(
            Slot::new(1),
            vec![Resource::new(1, Outline::from(vec![Slot::new(1)]))],
        )
        .unwrap();

    assert!(matches!(Schedule::new(vec![ei1, ei2]), Err(_)));
}
