use crate::model::{
    resources::Resource,
    slots::{Outline, Slot},
};

#[test]
pub fn equality_returns_true_if_equals() {
    let r1: Resource = Resource::new(1, 2, Outline::new());
    let r2: Resource = Resource::new(1, 2, Outline::new());

    assert!(r1 == r2);
}

#[test]
pub fn inequality_returns_true_if_not_equals() {
    let r1: Resource = Resource::new(1, 2, Outline::new());
    let r2: Resource = Resource::new(1, 320, Outline::new());

    assert!(r1 != r2);
}

#[test]
pub fn equality_returns_true_if_equals_omitting_outline() {
    let mut outline: Outline = Outline::new();
    outline.extend_from_slice(&[Slot::new(1)]).unwrap();

    let r1: Resource = Resource::new(1, 2, Outline::new());
    let r2: Resource = Resource::new(1, 2, outline);

    assert!(r1 == r2);
}
