// #[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
// #[macro_export]
// macro_rules! property {
//     (into, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Box<Self>
// {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> &T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_ref, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> &T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Box<Self>
// {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, box_self, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Box<Self> where $($where)* {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> &T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_ref, box_self, $name:ident, $t:ty , where { $($where:tt)* } )
// => {         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> &T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Box<Self> where $($where)* {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     ($name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> $t;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> $t;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Box<Self> {
//                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> &$t;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_ref, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> &$t;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Box<Self> {
//                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     ($name:ident, $t:ty, where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> $t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (box_self, $name:ident, $t:ty, where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> $t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Box<Self> where
// $($where)* {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> &$t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_ref, box_self, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> &$t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Box<Self> where
// $($where)* {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<&T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, ret_ref, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<&T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Box<Self>
// {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<T> where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, box_self, $name:ident, $t:ty , where { $($where:tt)* } )
// => {         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<T> where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Box<Self> where $($where)* {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } )
// => {         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<&T> where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, ret_ref, box_self, $name:ident, $t:ty , where {
// $($where:tt)* } ) => {         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<&T> where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<&$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, ret_ref, box_self, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<&$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, box_self, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<&$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, ret_ref, box_self, $name:ident, $t:ty , where { $($where:tt)* }
// ) => {         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<&$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 let mut clone = self;
//                 clone.[<set_ $name>](value);
//                 Box::new(clone)
//             }
//         }
//     };
//
//     // ordered variants (4 options)
//
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     (box_self, ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // ordered variants (3 options)
//
//     // When 'ret_ref' is missing, use (into, ret_opt, box_self)
//     (into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'ret_opt' is missing, use (into, ret_ref, box_self)
//     (into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'into' is missing, use (ret_opt, ret_ref, box_self)
//     (ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'box_self' is missing, use (into, ret_opt, ret_ref)
//     (into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//
//     // ordered variants (2 options)
//
//     // When 'box_self' and 'ret_ref' are missing, use (into, ret_opt)
//     (ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//     (into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//
//     // When 'box_self' and 'ret_opt' are missing, use (into, ret_ref)
//     (into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//
//     // When 'ret_ref' and 'ret_opt' are missing, use (into, box_self)
//     (into, box_self, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//     (box_self, into, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'ret_ref' are missing, use (ret_opt, box_self)
//     (ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'box_self' are missing, use (ret_opt, ret_ref)
//     (ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//
//     // When 'into' and 'ret_opt' are missing, use (ret_ref, box_self)
//     (ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
// }
//
// #[cfg(all(feature = "impl_with", feature = "getter_prefix"))]
// #[macro_export]
// macro_rules! property {
//     (into, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> &T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> &T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T)
// -> Self where $($where)* {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     ($name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> $t;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> &$t;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     ($name:ident, $t:ty, where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> $t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_ref, $name:ident, $t:ty, where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> &$t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<&T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//             fn [<with_ $name>]<T: Into<$t>>(mut self, value: T) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<T> where
// $($where)*;             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T)
// where $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self,
// value: T) -> Self where $($where)* {                 self.[<set_
// $name>](value);                 self
//             }
//         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } )
// => {         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<&T> where
// $($where)*;             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T)
// where $($where)*;             fn [<with_ $name>]<T: Into<$t>>(mut self,
// value: T) -> Self where $($where)* {                 self.[<set_
// $name>](value);                 self
//             }
//         }
//     };
//     (ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<&$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//             fn [<with_ $name>](mut self, value: $t) -> Self {
//                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<&$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//             fn [<with_ $name>](mut self, value: $t) -> Self where $($where)*
// {                 self.[<set_ $name>](value);
//                 self
//             }
//         }
//     };
//
//     // ordered variants (4 options)
//
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     (box_self, ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // ordered variants (3 options)
//
//     // When 'ret_ref' is missing, use (into, ret_opt, box_self)
//     (into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'ret_opt' is missing, use (into, ret_ref, box_self)
//     (into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'into' is missing, use (ret_opt, ret_ref, box_self)
//     (ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'box_self' is missing, use (into, ret_opt, ret_ref)
//     (into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//
//     // ordered variants (2 options)
//
//     // When 'box_self' and 'ret_ref' are missing, use (into, ret_opt)
//     (ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//     (into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//
//     // When 'box_self' and 'ret_opt' are missing, use (into, ret_ref)
//     (into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//
//     // When 'ret_ref' and 'ret_opt' are missing, use (into, box_self)
//     (into, box_self, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//     (box_self, into, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'ret_ref' are missing, use (ret_opt, box_self)
//     (ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'box_self' are missing, use (ret_opt, ret_ref)
//     (ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//
//     // When 'into' and 'ret_opt' are missing, use (ret_ref, box_self)
//     (ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
// }
//
// #[cfg(all(not(feature = "impl_with"), not(feature = "getter_prefix")))]
// #[macro_export]
// macro_rules! property {
//     (into, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> &T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;         }
//     };
//     (into, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> &T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;         }
//     };
//     ($name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> $t;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     (ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> &$t;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     ($name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> $t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//     (ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> &$t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (ret_ref, into, ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<&T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<T> where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } )
// => {         $crate::auto_props_paste! {
//             fn $name<T: Into<$t>>(&self) -> Option<&T> where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;         }
//     };
//     (ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<&$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     (ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn $name(&self) -> Option<&$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//
//     // ordered variants (4 options)
//
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     (box_self, ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // ordered variants (3 options)
//
//     // When 'ret_ref' is missing, use (into, ret_opt, box_self)
//     (into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'ret_opt' is missing, use (into, ret_ref, box_self)
//     (into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'into' is missing, use (ret_opt, ret_ref, box_self)
//     (ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'box_self' is missing, use (into, ret_opt, ret_ref)
//     (into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//
//     // ordered variants (2 options)
//
//     // When 'box_self' and 'ret_ref' are missing, use (into, ret_opt)
//     (ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//     (into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//
//     // When 'box_self' and 'ret_opt' are missing, use (into, ret_ref)
//     (into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//
//     // When 'ret_ref' and 'ret_opt' are missing, use (into, box_self)
//     (into, box_self, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//     (box_self, into, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'ret_ref' are missing, use (ret_opt, box_self)
//     (ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'box_self' are missing, use (ret_opt, ret_ref)
//     (ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//
//     // When 'into' and 'ret_opt' are missing, use (ret_ref, box_self)
//     (ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
// }
//
// #[cfg(all(not(feature = "impl_with"), feature = "getter_prefix"))]
// #[macro_export]
// macro_rules! property {
//     (into, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> &T;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;         }
//     };
//     (into, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> &T where $($where)*;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T) where
// $($where)*;         }
//     };
//     ($name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> $t;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     (ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> &$t;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     ($name:ident, $t:ty, where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> $t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//     (ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> &$t where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<&T>;
//             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T);
//         }
//     };
//     (into, ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<T> where
// $($where)*;             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T)
// where $($where)*;         }
//     };
//     (into, ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } )
// => {         $crate::auto_props_paste! {
//             fn [<get_ $name>]<T: Into<$t>>(&self) -> Option<&T> where
// $($where)*;             fn [<set_ $name>]<T: Into<$t>>(&mut self, value: T)
// where $($where)*;         }
//     };
//     (ret_opt, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<&$t>;
//             fn [<set_ $name>](&mut self, value: $t);
//         }
//     };
//     (ret_opt, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//     (ret_opt, ret_ref, $name:ident, $t:ty , where { $($where:tt)* } ) => {
//         $crate::auto_props_paste! {
//             fn [<get_ $name>](&self) -> Option<&$t> where $($where)*;
//             fn [<set_ $name>](&mut self, value: $t) where $($where)*;
//         }
//     };
//
//     // ordered variants (4 options)
//
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     (box_self, ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (into, ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // ordered variants (3 options)
//
//     // When 'ret_ref' is missing, use (into, ret_opt, box_self)
//     (into, ret_opt, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//     (into, box_self, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'ret_opt' is missing, use (into, ret_ref, box_self)
//     (into, ret_ref, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, into, box_self, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (into, box_self, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, into, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'into' is missing, use (ret_opt, ret_ref, box_self)
//     (ret_opt, ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_opt, box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (ret_ref, box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, box_self, $($rest)*);
//     };
//
//     // When 'box_self' is missing, use (into, ret_opt, ret_ref)
//     (into, ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (into, ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_opt, ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, ret_ref, $($rest)*);
//     };
//
//     // ordered variants (2 options)
//
//     // When 'box_self' and 'ret_ref' are missing, use (into, ret_opt)
//     (ret_opt, into, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//     (into, ret_opt, $($rest:tt)*) => {
//         property!(into, ret_opt, $($rest)*);
//     };
//
//     // When 'box_self' and 'ret_opt' are missing, use (into, ret_ref)
//     (into, ret_ref, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//     (ret_ref, into, $($rest:tt)*) => {
//         property!(into, ret_ref, $($rest)*);
//     };
//
//     // When 'ret_ref' and 'ret_opt' are missing, use (into, box_self)
//     (into, box_self, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//     (box_self, into, $($rest:tt)*) => {
//         property!(into, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'ret_ref' are missing, use (ret_opt, box_self)
//     (ret_opt, box_self, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//     (box_self, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, box_self, $($rest)*);
//     };
//
//     // When 'into' and 'box_self' are missing, use (ret_opt, ret_ref)
//     (ret_opt, ret_ref, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//     (ret_ref, ret_opt, $($rest:tt)*) => {
//         property!(ret_opt, ret_ref, $($rest)*);
//     };
//
//     // When 'into' and 'ret_opt' are missing, use (ret_ref, box_self)
//     (ret_ref, box_self, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
//     (box_self, ret_ref, $($rest:tt)*) => {
//         property!(ret_ref, box_self, $($rest)*);
//     };
// }
