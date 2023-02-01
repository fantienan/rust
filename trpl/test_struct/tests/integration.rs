use test_struct;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_struct::add(2,2))
}