//! Crate `inspector` extends popular data structures (such as `Option` and `Result`)
//! with additional methods for inspecting their payload. It is inspired by the `Iterator::inspect`.
//! Since no such methods are available by default on `Option` and `Result` types, this crate
//! implements a new traits for these types, which augment the respective types with various
//! inspection capabilities.

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
