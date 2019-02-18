#[cfg(all(test, feature = "futures"))]
mod tests {
    use futures::future::{empty, err, ok, Future};
    use inspector::FutureInspector;

    #[test]
    fn future_not_ready() {
        let mut inspect_called = false;

        let f = empty::<(), ()>();

        let _ = f.inspect_err(|_| inspect_called = true).poll();

        assert!(!inspect_called);
    }

    #[test]
    fn future_ready() {
        let mut inspect_called = false;

        let f = ok::<_, ()>(());

        let _ = f.inspect_err(|_| inspect_called = true).wait();

        assert!(!inspect_called);
    }

    #[test]
    fn future_err() {
        let mut inspect_called = false;

        let f = err::<(), _>(0);

        let _ = f
            .inspect_err(|e| {
                inspect_called = true;
                assert_eq!(*e, 0);
            })
            .wait();

        if cfg!(any(debug_assertions, feature = "inspect-release")) {
            assert!(inspect_called);
        } else {
            assert!(!inspect_called);
        }
    }
}
