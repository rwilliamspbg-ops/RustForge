use std::fmt::Display;

fn describe<T: Display>(value: T) -> String {
    format!("value: {value}")
}

fn main() {
    assert_eq!(describe(42), "value: 42");
    assert_eq!(describe("hi"), "value: hi");
}
