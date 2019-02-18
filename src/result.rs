use std::fmt;

/// `ResultInspector` makes it easier to examine the content of the `Result` object.
#[allow(clippy::module_name_repetitions)]
pub trait ResultInspector<T, E> {
    /// Do something with `Result`'s item, passing the value on.
    ///
    /// When using `Result`, you'll often chain several combinators together.
    /// While working on such code, you might want to check out what's happening
    /// at various parts in the pipeline. To do that, insert a call to inspect().
    ///
    /// It's more common for inspect() to be used as a debugging tool than to exist
    /// in your final code, but applications may find it useful in certain situations
    /// when data needs to be logged before being manipulated.
    /// See also `std::iter::Iterator::inspect`
    ///
    /// `ResultInspector::inspect()` only acts on the `Ok(_)` variant of the `Result` object
    /// and does nothing when it is `Err(_)`.
    ///
    /// ```rust
    /// # extern crate log;
    /// # use log::info;
    /// # use inspector::ResultInspector;
    ///
    /// let env = std::env::var("XXX").inspect(|env| info!("Env var XXX is {:?}", env));
    /// ```
    fn inspect<F>(self, f: F) -> Result<T, E>
    where
        F: FnMut(&T);

    /// Same as `ResultInspector::inspect()`, but only acts on `Err(_)` variant.
    ///
    /// ```rust
    /// # extern crate log;
    /// # use log::error;
    /// # use inspector::ResultInspector;
    ///
    /// let env = std::env::var("XXX")
    ///     .inspect_err(|err| error!("Failed to get env var XXX: {:?}", err))
    ///     .unwrap_or_else(|_| String::new());
    /// ```
    fn inspect_err<F>(self, f: F) -> Result<T, E>
    where
        F: FnMut(&E);

    /// Convenience wrapper for having a quick debug print out of your item.
    /// It is equivalent to calling `inspect(|item| println!("{:?}", item))`.
    ///
    /// ```rust
    /// # use inspector::ResultInspector;
    ///
    /// let env = std::env::var("XXX").debug();
    /// ```
    fn debug(self) -> Result<T, E>;
}

impl<T, E> ResultInspector<T, E> for Result<T, E>
where
    T: fmt::Debug,
{
    #[inline]
    fn inspect<F>(self, mut f: F) -> Self
    where
        F: FnMut(&T),
    {
        if cfg!(any(debug_assertions, feature = "inspect-release")) {
            if let Ok(ref item) = self {
                f(item);
            }
        }

        self
    }

    #[inline]
    fn inspect_err<F>(self, mut f: F) -> Self
    where
        F: FnMut(&E),
    {
        if let Err(ref item) = self {
            f(item);
        }
        self
    }

    #[inline]
    fn debug(self) -> Self {
        self.inspect(|item| println!("{:?}", item))
    }
}
