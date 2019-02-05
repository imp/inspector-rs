//! Crate `inspector` extends popular data structures (such as `Option` and `Result`)
//! with additional methods for inspecting their payload. It is inspired by the `Iterator::inspect`.
//! Since no such methods are available by default on `Option` and `Result` types, this crate
//! implements a new traits for these types, which augment the respective types with various
//! inspection capabilities.
//!
//! Implementation and availability of each trait is guarded by the dedicated feature, so that
//! you can choose which one is available.
//!
//! # Features
//! - `option` - enables trait `OptionInspector`
//! - `result` - enables trait `ResultInspector`
//! - `iter` - enables trait `IterInspector` (broken at the moment)
//! - `futures` - enables trait `FuturesInspector`

#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(rust_2018_idioms)]
#![warn(clippy::use_self)]
#![deny(warnings, missing_debug_implementations)]

#[cfg(feature = "futures")]
mod future;
#[cfg(feature = "iter")]
mod iter;
#[cfg(feature = "option")]
mod option;
#[cfg(feature = "result")]
mod result;

#[cfg(feature = "futures")]
pub use crate::future::Inspector as FuturesInspector;
#[cfg(feature = "iter")]
pub use crate::iter::Inspector as IterInspector;
#[cfg(feature = "option")]
pub use crate::option::Inspector as OptionInspector;
#[cfg(feature = "result")]
pub use crate::result::Inspector as ResultInspector;
