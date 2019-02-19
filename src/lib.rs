//! Crate `inspector` extends popular data structures (such as `Option` and `Result`)
//! with additional methods for inspecting their payload. It is inspired by the `Iterator::inspect`.
//! Since no such methods are available by default on `Option` and `Result` types, this crate
//! implements a new traits for these types, which augment the respective types with various
//! inspection capabilities.
//!
//! Implementation and availability of each trait is guarded by the dedicated feature, so that
//! you can choose which one is available. Sometimes you want these only for debug purposes, but
//! prefer to always leave the code in place. Feature `debug-only` helps in this case.
//! If enabled and compiled in `release` mode the combinators become effectively NOP.
//! This feature does nothing in `debug` mode.
//!
//! # Features
//! - `debug-only` - turnes the combinators into NOP in release mode
//! - `option` - enables trait `OptionInspector`
//! - `result` - enables trait `ResultInspector`
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
pub use crate::future::FutureInspector;
#[cfg(feature = "iter")]
pub use crate::iter::IterInspector;
#[cfg(feature = "option")]
pub use crate::option::OptionInspector;
#[cfg(feature = "result")]
pub use crate::result::ResultInspector;
