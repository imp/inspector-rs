use super::Inspector;

impl<T, E> Inspector for Result<T, E>
where
    T: Sized,
    E: Sized,
{
    type ItemOk = T;
    type ItemErr = E;

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
        if let Ok(ref item) = self {
            f(item);
        }
        self
    }

    fn inspect_err<F>(self, mut f: F) -> Self
    where
        F: FnMut(&Self::ItemErr),
    {
        if let Err(ref item) = self {
            f(item);
        }
        self
    }

    fn debug(self) -> Self {
        unimplemented!()
    }

    fn display(self) -> Self {
        unimplemented!()
    }
}
