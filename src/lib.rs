// pub use paste::paste as auto_props_paste;

// mod original;

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
