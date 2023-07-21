use crate::model::slots::{Outline, Slot};

#[test]
pub fn add_slot_succeeds_with_unique_slots() {
    let mut outline: Outline = Outline::new();
    let mut results: Vec<Result<(), ()>> = vec![];

    for i in 0..100 {
        results.push(outline.add_slot(Slot::new(i)));
    }

    assert!(results.iter().all(|r| r.is_ok()));
}

#[test]
pub fn add_slot_fails_with_non_unique_slots() {
    let mut outline: Outline = Outline::new();
    let mut results: Vec<Result<(), ()>> = vec![];

    for i in 0..100 {
        results.push(outline.add_slot(Slot::new(i)));
    }

    results.push(outline.add_slot(Slot::new(1)));

    assert!(!results.iter().all(|r| r.is_ok()));
}

#[test]
pub fn extend_from_slice_succeeds_with_unique_slots() {
    let mut outline: Outline = Outline::new();
    let mut results: Vec<Result<(), ()>> = vec![];

    for i in 0..100 {
        results.push(outline.add_slot(Slot::new(i)));
    }

    assert!(matches!(
        outline.extend_from_slice(&[Slot::new(200)]),
        Ok(_)
    ));
}

#[test]
pub fn extend_from_slice_fails_with_non_unique_slots() {
    let mut outline: Outline = Outline::new();
    let mut results: Vec<Result<(), ()>> = vec![];

    for i in 0..100 {
        results.push(outline.add_slot(Slot::new(i)));
    }

    assert!(matches!(outline.extend_from_slice(&[Slot::new(1)]), Err(_)));
}
