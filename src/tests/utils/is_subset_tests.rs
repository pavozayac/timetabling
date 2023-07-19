use crate::utils::is_subset;

#[test]
pub fn returns_true_given_correct_primitive_slices() {
    let slice = &[1, 2, 3];
    let big_slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

    assert_eq!(true, is_subset(big_slice, slice));
}

#[test]
pub fn returns_false_given_incorrect_primitive_slices() {
    let slice = &[1, 2, 3, 100];
    let big_slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

    assert_eq!(false, is_subset(big_slice, slice));
}
