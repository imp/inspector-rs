use std::fmt;

/// `OptionInspector` makes it easier to examine the content of the `Option` object.
#[allow(clippy::module_name_repetitions)]
pub trait OptionInspector<T> {
    /// Do something with `Option`'s item, passing the value on.
    ///
    /// When using `Option`, you'll often chain several combinators together.
    /// While working on such code, you might want to check out what's happening
    /// at various parts in the pipeline. To do that, insert a call to inspect().
    ///
    /// It's more common for inspect() to be used as a debugging tool than to exist
    /// in your final code, but applications may find it useful in certain situations
    /// when data needs to be logged before being manipulated.
    /// See also `std::iter::Iterator::inspect`
    ///
    /// ```rust
    /// # extern crate log;
    /// # use log::info;
    /// # use inspector::OptionInspector;
    ///
    /// let env = std::env::var_os("XXX").inspect(|env| info!("Env var XXX is {:?}", env));
    /// ```
    fn inspect<F>(self, f: F) -> Option<T>
    where
        F: FnMut(&T);

    /// Convenience wrapper for having a quick debug print out of your item.
    /// It is equivalent to calling `inspect(|item| println!("{:?}", item))`.
    ///
    /// ```rust
    /// # use inspector::OptionInspector;
    ///
    /// let env = std::env::var_os("XXX").debug();
    /// ```
    fn debug(self) -> Option<T>;
}

impl<T> OptionInspector<T> for Option<T>
where
    T: fmt::Debug,
{
    #[inline]
    fn inspect<F>(self, mut f: F) -> Self
    where
        F: FnMut(&T),
    {
        if cfg!(any(debug_assertions, feature = "inspect-release")) {
            if let Some(ref item) = self {
                f(item)
            }
        }

        self
    }

    #[inline]
    fn debug(self) -> Self {
        self.inspect(|item| println!("{:?}", item))
    }
}
