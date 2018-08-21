extern crate inspector;

use inspector::Inspector;

#[test]
fn result_ok() {
    let mut inspect_called = false;

    let o: Result<u8, u8> = Ok(2);

    o.inspect(|i| {
        inspect_called = true;
        assert_eq!(*i, Ok(2))
    }).unwrap();

    assert!(inspect_called);

    inspect_called = false;

    o.inspect_ok(|i| {
        inspect_called = true;
        assert_eq!(*i, 2)
    }).unwrap();

    assert!(inspect_called);

    o.inspect_err(|_| assert!(false)).unwrap();
}

#[test]
fn result_err() {
    let mut inspect_called = false;

    let o: Result<u8, u8> = Err(3);

    o.inspect(|i| {
        inspect_called = true;
        assert_eq!(*i, Err(3))
    }).unwrap_err();

    assert!(inspect_called);

    inspect_called = false;

    o.inspect_err(|i| {
        inspect_called = true;
        assert_eq!(*i, 3)
    }).unwrap_err();

    assert!(inspect_called);

    o.inspect_ok(|_| assert!(false)).unwrap_err();
}

#[test]
fn borrowed() {
    let o = Some(2);
    let b = &o;
    b.inspect(|i| assert_eq!(*i, Some(2)));
    o.inspect_ok(|i| assert_eq!(*i, 2));
}
