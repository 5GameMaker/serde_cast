//! Cast one struct into another
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_cast::serde_cast;
//!
//! #[derive(Serialize)]
//! struct A {
//!     field: String,
//! }
//!
//! #[derive(Deserialize)]
//! struct B {
//!     field: String,
//! }
//!
//! let a = A { field: "This was a mistake".to_string() };
//! let b: B = serde_cast!(&a);
//!
//! assert_eq!(a.field, b.field);
//! ```

#[doc(hidden)]
pub use serde as __ser;
#[doc(hidden)]
pub use ron as __de;

/// The macro.
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use serde_cast::serde_cast;
///
/// #[derive(Serialize)]
/// struct A {
///     field: String,
/// }
///
/// #[derive(Deserialize)]
/// struct B {
///     field: String,
/// }
///
/// let a = A { field: "This was a mistake".to_string() };
/// let b: B = serde_cast!(&a);
///
/// assert_eq!(a.field, b.field);
/// ```
///
/// Casting A to B if fields of A and B don't exactly match is undefined behavior
#[macro_export]
macro_rules! serde_cast {
    ($expr:expr) => {
        {
            use $crate::__ser::{Serialize, Deserialize};
            $crate::__de::from_str(&$crate::__de::to_string($expr).expect("serde_cast failed")).expect("serde_cast failed")
        }
    };
}
