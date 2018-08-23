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

#![cfg_attr(
    all(feature = "cargo-clippy", feature = "pedantic"),
    warn(clippy_pedantic)
)]
#![cfg_attr(feature = "cargo-clippy", warn(use_self))]
#![deny(warnings, missing_debug_implementations)]

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
