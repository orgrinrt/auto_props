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
//!
//! # Allocation
//!
//! Both positions are features, and neither changes what this crate does. It is
//! `macro_rules!` forwarding to a proc macro, and what that proc macro writes is trait
//! method declarations naming only the types the caller named, so nothing it produces
//! reaches for `std` or an allocator on its own account.
//!
//! | Feature | Effect |
//! |---|---|
//! | `no_std` | Adds `#![no_std]`. |
//! | `no_alloc` | Implies `no_std`. States what is already true. |
//!
//! They exist so a consumer whose workspace turns them on everywhere can name them, and
//! so the claim is checked rather than believed: `tests/feature_matrix.rs` compiles a
//! `#![no_std]` consumer whose trait is built out of [`property!`].
//!
//! The proc-macro crate is deliberately not carried along. It runs on the host inside the
//! compiler, where syn, quote and proc-macro2 all use `std`, so it cannot be `no_std`
//! whatever this crate declares.

#![cfg_attr(feature = "no_std", no_std)]

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
