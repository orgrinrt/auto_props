//! Accessor declarations for a trait, written once.
//!
//! [`property!`] expands to a getter and a setter as trait declarations, which an
//! implementor fills in, plus a `with_*` builder that arrives with a body already. What
//! varies is how the setter's parameter type and the getter's return type relate to the
//! property type, which the specification writes as `<parameter> -> <return>` with `_`
//! standing for the property type on each side.
//!
//! ```
//! use auto_props::property;
//!
//! trait Named {
//!     property!(name: String);
//!     property!(title: Into<String>);
//!     property!(count: u32 = _ -> _);
//!     property!(label: String as Option<String>);
//! }
//! ```
//!
//! - `name: String` takes and returns `String`.
//! - `title: Into<String>` makes the setter generic over anything that converts.
//! - `count: u32 = _ -> _` is the same as the bare form, spelled out.
//! - `label: String as Option<String>` changes the return and leaves the parameter.

pub use auto_props_proc_macros::common as __common;

#[macro_export]
macro_rules! property {
    ($name:ident: $ty:ty) => {
        $crate::__common!($name: $ty = _ -> _);
    };
    ($name:ident: $ty:ty = $($rest:tt)+) => {
        $crate::__common!($name: $ty = $($rest)+);
    };
    ($name:ident: $ty:ty as $($rest:tt)+) => {
        $crate::__common!($name: $ty = _ -> $($rest)+);
    };
    ($name:ident: $ty:ty where { $($rest:tt)+ }) => {
        $crate::__common!($name: $ty = _ -> _ where { $($rest)* });
    };
}
