//#[macro_use]
//extern crate dont_panic;

#[cfg(feature = "iter")]
mod iter;
#[cfg(feature = "option")]
mod option;
#[cfg(feature = "result")]
mod result;

#[cfg(feature = "iter")]
pub use iter::Inspector as IterInspector;
#[cfg(feature = "option")]
pub use option::Inspector as OptionInspector;
#[cfg(feature = "result")]
pub use result::Inspector as ResultInspector;


pub trait MasterInspector {
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
