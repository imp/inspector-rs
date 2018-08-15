#[macro_use]
extern crate dont_panic;

pub trait Inspector {
    type ItemOk: Sized;
    type ItemErr: Sized;

    fn inspect<F>(self, f: F) -> Self
    where
        F: FnMut(&Self);

    fn inspect_ok<F>(self, f: F) -> Self
    where
        F: FnMut(&Self::ItemOk);

    fn inspect_err<F>(self, f: F) -> Self
    where
        F: FnMut(&Self::ItemErr);
}

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
}

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
}
