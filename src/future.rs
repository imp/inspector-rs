//use std::fmt;

use futures::{Async, Future, Poll};

/// This is created by the `FutureInspector::inspect_err` method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct InspectErr<A, F>
where
    A: Future,
{
    future: A,
    f: Option<F>,
}

impl<A, F> InspectErr<A, F>
where
    A: Future,
    for<'r> F: FnOnce(&'r A::Error) -> (),
{
    fn new(future: A, f: F) -> Self {
        Self { future, f: Some(f) }
    }
}

impl<A, F> Future for InspectErr<A, F>
where
    A: Future,
    F: FnOnce(&A::Error),
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<A::Item, A::Error> {
        match self.future.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(e)) => Ok(Async::Ready(e)),
            Err(e) => {
                if cfg!(any(debug_assertions, not(feature = "debug-only"))) {
                    (self.f.take().expect("cannot poll InspectErr twice"))(&e);
                }
                Err(e)
            }
        }
    }
}

/// Do something with the error of a future, passing it on.
/// This combinator was implemented in futures-0.2 series (which is now dead),
/// but never made it back to futures-0.1.x series.
#[allow(clippy::module_name_repetitions)]
pub trait FutureInspector<I, E>: Future<Item = I, Error = E> {
    /// Do something with the error of a future, passing it on.
    ///
    /// When using futures, you'll often chain several of them together.
    /// While working on such code, you might want to check out what's happening to the errors
    /// at various parts in the pipeline. To do that, insert a call to `inspect_err()`.

    fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>
    where
        for<'r> F: FnOnce(&'r Self::Error) -> (),
        Self: Sized,
    {
        assert_future::<Self::Item, Self::Error, _>(InspectErr::new(self, f))
    }

    // fn debug(self) -> future::Inspect<Self, _>;
}

impl<I, E, T> FutureInspector<I, E> for T where T: Future<Item = I, Error = E> {}

fn assert_future<A, B, F>(f: F) -> F
where
    F: Future<Item = A, Error = B>,
{
    f
}
