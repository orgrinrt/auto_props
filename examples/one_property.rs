//! The smallest thing the macro does: three accessors from one line in a trait.
//!
//! `property!(name: String)` inside a trait writes three items into it. `name()` and
//! `set_name()` are declarations an implementor fills in, because only the implementor knows
//! where the value lives. `with_name()` arrives with a body already, because the body is the
//! same every time: set it and hand `Self` back.
//!
//! That third one is what the macro saves you writing, and it is the one people forget on the
//! fourth property of a trait that has nine.

use auto_props::property;

trait Named {
    property!(name: String);
}

struct Person {
    name: String,
}

// Two methods, not three. `with_name` is not written here and is not missing.
impl Named for Person {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

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
