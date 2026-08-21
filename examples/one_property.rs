//! The smallest thing the macro does: three accessors from one line in a trait.
//!
//! `property!(name: String)` inside a trait writes three items into it. `name()` and
//! `set_name()` are declarations an implementor fills in, because only the implementor knows
//! where the value lives. `with_name()` arrives with a body already, because the body is the
//! same every time: set it and hand `Self` back.
//!
//! That third one is what the macro saves you writing, and it is the one people forget on the
//! fourth property of a trait that has nine.

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
use auto_props::property;

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
trait Named {
    property!(name: String);
}

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
struct Person {
    name: String,
}

// Two methods, not three. `with_name` is not written here and is not missing.
#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
impl Named for Person {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

// Written against the default selection. `getter_prefix` renames every getter to `get_*` and
// `impl_with` is what writes the `with_*` builder at all, so under either change the code
// below names methods the trait does not declare. cargo builds every example under every
// feature selection, so the gate is here; `tests/feature_matrix.rs` covers the other
// selections, and an example showing all of them would be showing the features rather than
// the macro.
#[cfg(any(not(feature = "impl_with"), feature = "getter_prefix"))]
fn main() {
    println!("this example is written against the default getter naming");
}

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
fn main() {
    let mut person = Person {
        name: String::from("before"),
    };

    person.set_name(String::from("after"));
    println!("set:   {}", person.name());

    let built = Person {
        name: String::from("before"),
    }
    .with_name(String::from("built"));
    println!("built: {}", built.name());
}
