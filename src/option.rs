use std::fmt;

pub trait Inspector<T> {
    fn inspect<F>(self, f: F) -> Option<T>
    where
        F: FnMut(&T);

    fn debug(self) -> Option<T>;
}

impl<T> Inspector<T> for Option<T>
where
    T: fmt::Debug,
{
    fn inspect<F>(self, mut f: F) -> Self
    where
        F: FnMut(&T),
    {
        if let Some(ref item) = self {
            f(item)
        }
        self
    }

    fn debug(self) -> Self {
        self.inspect(|item| println!("{:?}", item))
    }
}
