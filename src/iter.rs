use std::fmt;
use std::iter::Iterator;
//use std::ops;

pub trait IterInspector: Iterator {
    fn debug(self) -> DebugInspector<Self> {
        unimplemented!()
    }
    fn display(self) -> DisplayInspector<Self> {
        unimplemented!()
    }
}

impl<T> IterInspector for T
where
    T: Iterator,
    T::Item: fmt::Debug,
{
    fn debug(self) -> DebugInspector<Self> {
        let iter = self.inspect(|elt| println!("{:?}", elt));
        Self { iter }
    }
}

impl<T> IterInspector for T
where
    T: Iterator,
    T::Item: fmt::Display,
{
    fn display(self) -> DebugInspector<Self> {
        let iter = self.inspect(|elt| println!("{}", elt));
        Self { iter }
    }
}

impl<T> IterInspector for T
where
    T: Iterator,
    T::Item: fmt::Debug + fmt::Display,
{
    fn debug(self) -> DebugInspector<Self> {
        let iter = self.inspect(|elt| println!("{:?}", elt));
        Self { iter }
    }

    fn display(self) -> DebugInspector<Self> {
        let iter = self.inspect(|elt| println!("{}", elt));
        Self { iter }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
struct DebugInspector<I> {
    iter: I,
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
struct DisplayInspector<I> {
    iter: I,
}

impl<I> fmt::Debug for DebugInspector<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DebugInspector")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I> fmt::Debug for DisplayInspector<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DisplayInspector")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I> Iterator for DebugInspector<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    //    #[inline]
    //    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
    //        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: ops::Try<Ok=Acc>
    //    {
    //        let f = &mut self.f;
    //        self.iter.try_fold(init, move |acc, item| { f(&item); fold(acc, item) })
    //    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.fold(init, move |acc, item| {
            f(&item);
            fold(acc, item)
        })
    }
}
