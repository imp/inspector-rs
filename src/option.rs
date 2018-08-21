use super::Inspector;

impl<T> Inspector for Option<T>
where
    T: Sized,
{
    type ItemOk = T;
    type ItemErr = ();

    fn inspect<F>(self, mut f: F) -> Self
    where
        F: FnMut(&Self),
    {
        f(&self);
        self
    }

    fn inspect_ok<F>(self, mut f: F) -> Self
    where
        F: FnMut(&Self::ItemOk),
    {
        if let Some(ref item) = self {
            f(item);
        }
        self
    }

    fn inspect_err<F>(self, mut _f: F) -> Self
    where
        F: FnMut(&Self::ItemErr),
    {
        dont_panic!();
    }

    fn debug(self) -> Self {
        unimplemented!()
    }

    fn display(self) -> Self {
        unimplemented!()
    }
}
