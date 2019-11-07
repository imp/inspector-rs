extern crate inspector;

#[cfg(all(test, feature = "option"))]
mod tests {
    use inspector::OptionInspector;

    #[test]
    fn owned() {
        let o = Some(2);
        o.inspect(|i| assert_eq!(*i, 2));
    }

    #[test]
    fn borrowed() {
        let o = Some(2);
        let b = &o;
        b.inspect(|i| assert_eq!(*i, 2));
    }

    #[test]
    fn debug() {
        let _o = Some(155);
        let x = _o.debug().map(|i| i * 2);
        assert_eq!(x, Some(310));
    }
}
