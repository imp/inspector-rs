use std::fmt;

pub trait Inspector<T, E> {
    fn inspect<F>(self, f: F) -> Result<T, E>
    where
        F: FnMut(&T);

    fn inspect_err<F>(self, f: F) -> Result<T, E>
    where
        F: FnMut(&E);

    fn debug(self) -> Result<T, E>;
}

impl<T, E> Inspector<T, E> for Result<T, E>
where
    T: fmt::Debug,
{
    fn inspect<F>(self, mut f: F) -> Self
    where
        F: FnMut(&T),
    {
        if let Ok(ref item) = self {
            f(item);
        }
        self
    }

    fn inspect_err<F>(self, mut f: F) -> Self
    where
        F: FnMut(&E),
    {
        if let Err(ref item) = self {
            f(item);
        }
        self
    }

    fn debug(self) -> Self {
        self.inspect(|item| println!("{:?}", item))
    }
}
