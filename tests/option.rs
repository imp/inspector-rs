extern crate inspector;

use inspector::Inspector;

#[test]
fn owned() {
    let o = Some(2);
    o.inspect(|i| assert_eq!(*i, Some(2)));
    o.inspect_ok(|i| assert_eq!(*i, 2));
}

#[test]
fn borrowed() {
    let o = Some(2);
    let b = &o;
    b.inspect(|i| assert_eq!(*i, Some(2)));
    o.inspect_ok(|i| assert_eq!(*i, 2));
}
