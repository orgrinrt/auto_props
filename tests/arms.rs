//! Every arm of `property!`, and every part of the specification each arm expands into.
//!
//! The macro writes a getter and a setter as trait declarations, plus a `with_*` builder
//! that comes with a body. What varies between arms is how the setter's parameter type and
//! the getter's return type are derived from the property type, which the specification
//! writes as `<param> -> <return>` with `_` standing for the property type in each.

use auto_props::property;

// --- the bare arm ------------------------------------------------------------------------

trait Bare {
    property!(name: String);
}

struct Holder {
    name: String,
    count: u32,
}

impl Bare for Holder {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

#[test]
fn the_bare_arm_takes_and_returns_the_property_type() {
    let mut holder = Holder {
        name: "before".to_string(),
        count: 0,
    };
    holder.set_name("after".to_string());
    assert_eq!(holder.name(), "after");
}

#[test]
fn the_builder_arrives_with_a_body_and_returns_self() {
    let holder = Holder {
        name: "before".to_string(),
        count: 0,
    }
    .with_name("built".to_string());
    assert_eq!(holder.name, "built");
}

// --- the `=` arm, which spells out both sides ---------------------------------------------

/// A type with no `&`-of-it lint attached, so the borrowed arm can be shown as itself.
#[derive(Clone, PartialEq, Debug)]
pub struct Payload(String);

trait Spelled {
    property!(count: u32 = _ -> _);
    property!(borrowed: Payload = &_ -> _);
}

impl Spelled for Holder {
    fn count(&self) -> u32 {
        self.count
    }

    fn set_count(&mut self, value: u32) {
        self.count = value;
    }

    fn borrowed(&self) -> Payload {
        Payload(self.name.clone())
    }

    fn set_borrowed(&mut self, value: &Payload) {
        self.name = value.0.clone();
    }
}

#[test]
fn the_placeholder_stands_for_the_property_type_on_both_sides() {
    let mut holder = Holder {
        name: String::new(),
        count: 0,
    };
    holder.set_count(9);
    assert_eq!(holder.count(), 9);
}

#[test]
fn a_prefix_before_the_placeholder_reaches_the_setter_parameter() {
    let mut holder = Holder {
        name: String::new(),
        count: 0,
    };
    // The setter takes `&Payload` rather than `Payload`, which is what `&_` asked for.
    holder.set_borrowed(&Payload("borrowed".to_string()));
    assert_eq!(holder.borrowed(), Payload("borrowed".to_string()));
}

// --- the `as` arm, which changes only the return -------------------------------------------

trait Returned {
    property!(label: String as Option<String>);
}

impl Returned for Holder {
    fn label(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn set_label(&mut self, value: String) {
        self.name = value;
    }
}

#[test]
fn the_as_arm_changes_the_return_and_leaves_the_parameter() {
    let mut holder = Holder {
        name: String::new(),
        count: 0,
    };
    holder.set_label("labelled".to_string());
    assert_eq!(holder.label(), Some("labelled".to_string()));
}

// --- the `where` arm ------------------------------------------------------------------------

trait Bounded {
    property!(bounded: u32 where { Self: Sized });
}

impl Bounded for Holder {
    fn bounded(&self) -> u32 {
        self.count
    }

    fn set_bounded(&mut self, value: u32) {
        self.count = value;
    }
}

#[test]
fn the_where_arm_carries_the_bound_onto_the_accessors() {
    let mut holder = Holder {
        name: String::new(),
        count: 1,
    };
    holder.set_bounded(4);
    assert_eq!(holder.bounded(), 4);
}

// --- `Into<T>`, which makes the setter generic ----------------------------------------------

trait Converted {
    property!(title: Into<String>);
}

impl Converted for Holder {
    fn title<T: Into<String>>(&self) -> String {
        self.name.clone()
    }

    fn set_title<T: Into<String>>(&mut self, value: T) {
        self.name = value.into();
    }
}

#[test]
fn an_into_property_accepts_anything_that_converts() {
    let mut holder = Holder {
        name: String::new(),
        count: 0,
    };
    // `&str` rather than `String`, which is the point of declaring it as `Into`.
    holder.set_title("converted");
    assert_eq!(holder.name, "converted");
}

// --- the two regressions --------------------------------------------------------------------

/// A user type whose name begins with `Into` is not an `Into<T>` property.
///
/// The variant used to be detected with `starts_with("Into")` over the type's printed form,
/// and the inner type with `replace("Into", "")`, so this was silently rewritten to a
/// property of a type called `Thing`, which does not exist. `IntoIterator` and `IntoIter`
/// are the same shape and would have gone the same way.
#[derive(Clone, PartialEq, Debug)]
pub struct IntoThing(u8);

trait NotConverted {
    property!(thing: IntoThing);
}

struct ThingHolder(IntoThing);

impl NotConverted for ThingHolder {
    fn thing(&self) -> IntoThing {
        self.0.clone()
    }

    fn set_thing(&mut self, value: IntoThing) {
        self.0 = value;
    }
}

#[test]
fn a_type_merely_named_like_into_is_left_alone() {
    let mut holder = ThingHolder(IntoThing(1));
    holder.set_thing(IntoThing(2));
    assert_eq!(holder.thing(), IntoThing(2));
}

/// `Into<T>` where `T` itself carries arguments.
///
/// Stripping every angle bracket out of the printed form left `Vec u8`, which does not
/// parse, so the macro panicked rather than reporting anything.
trait NestedConversion {
    property!(bytes: Into<Vec<u8>>);
}

struct ByteHolder(Vec<u8>);

impl NestedConversion for ByteHolder {
    fn bytes<T: Into<Vec<u8>>>(&self) -> Vec<u8> {
        self.0.clone()
    }

    fn set_bytes<T: Into<Vec<u8>>>(&mut self, value: T) {
        self.0 = value.into();
    }
}

#[test]
fn an_into_of_a_generic_type_keeps_its_arguments() {
    let mut holder = ByteHolder(Vec::new());
    holder.set_bytes("bytes");
    assert_eq!(holder.0, b"bytes");
}

/// A return type whose name contains an underscore is not a placeholder.
///
/// The placeholder branch used to be chosen by asking whether the rest of the stream's
/// printed form contained `_` anywhere, and an identifier carrying one satisfies that. So a
/// return type named `My_Type` was taken for a placeholder specification, the whole
/// identifier was consumed as the prefix, and the parse then failed looking for the `_` that
/// was never there. The type has to sit in the *return* position to reach that check: in the
/// property-type position it is parsed as a type before the check happens, which is why an
/// earlier version of this test passed against the defect.
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug)]
pub struct My_Type(u8);

trait UnderscoreNamed {
    property!(named: u8 as My_Type);
}

struct NamedHolder(u8);

impl UnderscoreNamed for NamedHolder {
    fn named(&self) -> My_Type {
        My_Type(self.0)
    }

    fn set_named(&mut self, value: u8) {
        self.0 = value;
    }
}

#[test]
fn an_underscore_inside_an_identifier_is_not_a_placeholder() {
    let mut holder = NamedHolder(1);
    holder.set_named(7);
    assert_eq!(holder.named(), My_Type(7));
}
