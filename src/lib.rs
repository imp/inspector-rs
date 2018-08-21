#[macro_use]
extern crate dont_panic;

mod option;
mod result;

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

    fn debug(self) -> Self;

    fn display(self) -> Self;
}
