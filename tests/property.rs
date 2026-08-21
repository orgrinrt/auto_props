//! `property!` writes the accessor surface into a trait: the getter and setter are declarations
//! an implementor fills in, and the builder-style `with_*` comes with a body already, which is
//! the part the macro saves you writing.
//!
//! Written against the default naming. `getter_prefix` renames every getter to `get_*` and
//! `impl_with` is what writes the `with_*` builder at all, so under either change these
//! implementations would name methods the trait does not declare, and cargo builds a
//! test under whatever selection it is given. The whole file is gated rather than each impl,
//! because the naming is what the file is about; `tests/feature_matrix.rs` covers the
//! prefixed form.
#![cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]

use auto_props::property;

trait Named {
    property!(name: String);
}

struct Thing {
    name: String,
}

impl Named for Thing {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

#[test]
fn the_setter_and_getter_are_declared_by_the_macro() {
    let mut thing = Thing {
        name: "before".to_string(),
    };
    thing.set_name("after".to_string());
    assert_eq!(thing.name(), "after");
}

#[test]
fn the_builder_method_comes_with_a_body() {
    let thing = Thing {
        name: "before".to_string(),
    }
    .with_name("built".to_string());
    assert_eq!(thing.name(), "built");
}
