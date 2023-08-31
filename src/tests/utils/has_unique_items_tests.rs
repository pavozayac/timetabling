use crate::utils::has_unique_items;

#[test]
pub fn returns_true_given_unique_items() {
    let slice = &[1, 2, 3, 4, 5, 6];

    assert!(has_unique_items(slice));
}

#[test]
pub fn returns_false_given_non_unique_items() {
    let slice = &[1, 2, 3, 4, 1, 6];

    assert!(!has_unique_items(slice));
}

#[derive(PartialEq, Eq, Hash)]
struct TestStruct<'a> {
    string: &'a str,
    number: i64,
}

#[test]
pub fn returns_true_given_unique_structs() {
    let slice = &[
        TestStruct {
            string: "hello",
            number: 1,
        },
        TestStruct {
            string: "hello",
            number: 2,
        },
        TestStruct {
            string: "goodbye",
            number: 1000000,
        },
    ];

    assert!(has_unique_items(slice));
}

#[test]
pub fn returns_false_given_unique_structs() {
    let slice = &[
        TestStruct {
            string: "hello",
            number: 1,
        },
        TestStruct {
            string: "hello",
            number: 1,
        },
        TestStruct {
            string: "goodbye",
            number: 1000000,
        },
    ];

    assert!(!has_unique_items(slice));
}
