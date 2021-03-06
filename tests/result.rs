extern crate inspector;

#[cfg(all(test, feature = "result"))]
mod tests {
    use inspector::ResultInspector;

    #[test]
    fn result_ok() {
        let mut inspect_called = false;

        let o: Result<u8, u8> = Ok(2);

        o.inspect(|i| {
            inspect_called = true;
            assert_eq!(*i, 2)
        })
        .unwrap();

        if cfg!(any(debug_assertions, not(feature = "debug-only"))) {
            assert!(inspect_called);
        } else {
            assert!(!inspect_called);
        }

        o.inspect_err(|_| unreachable!()).unwrap();
    }

    #[test]
    fn result_err() {
        let mut inspect_called = false;

        let o: Result<u8, u8> = Err(3);

        o.inspect(|i| {
            inspect_called = true;
            assert_eq!(*i, 3)
        })
        .unwrap_err();

        assert!(!inspect_called);

        o.inspect_err(|i| {
            inspect_called = true;
            assert_eq!(*i, 3)
        })
        .unwrap_err();

        if cfg!(any(debug_assertions, not(feature = "debug-only"))) {
            assert!(inspect_called);
        } else {
            assert!(!inspect_called);
        }

        o.inspect(|_| unreachable!()).unwrap_err();
    }
}
