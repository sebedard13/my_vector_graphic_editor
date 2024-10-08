#![allow(dead_code)]
use macros::boxed;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Foo {
    a: i32,
    b: i32,
}

impl Foo {
    #[boxed]
    fn new(a: i32, b: i32) -> Foo {
        Foo { a, b }
    }
}

#[test]
fn boxed_test() {
    let foo = Foo::boxed(1, 2);
    let new_foo = Foo::new(1, 2);
    assert_eq!(*foo, new_foo);
}
