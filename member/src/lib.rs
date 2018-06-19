pub fn gimme_int() -> u32 {
    123
}

#[test]
fn gimme_int_should_be_123() {
    let int: u32 = gimme_int();
    assert_eq!(int, 123);
}
