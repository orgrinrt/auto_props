//! Each way of writing a property, and what changes between them.
//!
//! The macro derives two types from the property type: what the setter takes and what the
//! getter returns. The arms are the ways of saying what those should be, and the underscore
//! stands for the property type itself.
//!
//! | written as | setter takes | getter returns |
//! |---|---|---|
//! | `x: T` | `T` | `T` |
//! | `x: T = &_ -> _` | `&T` | `T` |
//! | `x: T as U` | `T` | `U` |
//! | `x: T where { .. }` | `T` | `T`, with the bound on each |
//! | `x: Into<T>` | generic over `Into<T>` | `T` |
//!
//! Only the first is shorthand for anything: `x: T` is `x: T = _ -> _` written short.

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
use auto_props::property;

/// A type worth borrowing rather than cloning, so the borrowed arm is shown doing its job.
#[derive(Clone, Debug, PartialEq)]
#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
struct Blob(Vec<u8>);

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
trait Surface {
    // Both sides are the property type.
    property!(count: u32);

    // Spelled out, and the setter borrows. `&_` is the property type behind a reference,
    // which is what you want for anything a caller would rather not hand over.
    property!(payload: Blob = &_ -> _);

    // Only the return changes. The value is always there; the getter says it might not be,
    // which is how an accessor models a field the trait treats as optional.
    property!(label: String as Option<String>);

    // The bound lands on every accessor the line writes, rather than on the trait.
    property!(bounded: u32 where { Self: Sized });

    // The setter becomes generic, so a caller passes anything that converts. This is the arm
    // that saves the most typing and the one whose expansion is least obvious.
    property!(title: Into<String>);
}

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
struct Record {
    count: u32,
    payload: Blob,
    label: String,
    title: String,
}

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
impl Surface for Record {
    fn count(&self) -> u32 {
        self.count
    }

    fn set_count(&mut self, value: u32) {
        self.count = value;
    }

    fn payload(&self) -> Blob {
        self.payload.clone()
    }

    fn set_payload(&mut self, value: &Blob) {
        self.payload = value.clone();
    }

    fn label(&self) -> Option<String> {
        Some(self.label.clone())
    }

    fn set_label(&mut self, value: String) {
        self.label = value;
    }

    fn bounded(&self) -> u32 {
        self.count
    }

    fn set_bounded(&mut self, value: u32) {
        self.count = value;
    }

    fn title<T: Into<String>>(&self) -> String {
        self.title.clone()
    }

    fn set_title<T: Into<String>>(&mut self, value: T) {
        self.title = value.into();
    }
}

#[cfg(all(feature = "impl_with", not(feature = "getter_prefix")))]
fn new_record() -> Record {
    Record {
        count: 0,
        payload: Blob(vec![]),
        label: String::from("none"),
        title: String::from("untitled"),
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
    let mut record = new_record();

    record.set_count(7);
    println!("count:   {}", record.count());

    // The setter borrows, so the blob is still the caller's afterwards.
    let blob = Blob(vec![1, 2, 3]);
    record.set_payload(&blob);
    println!(
        "payload: {:?} (and the caller still has {:?})",
        record.payload(),
        blob
    );

    record.set_label(String::from("shipped"));
    println!("label:   {:?}", record.label());

    record.set_bounded(9);
    println!("bounded: {}", record.bounded());

    // A `&str` where the property is a `String`, because the setter is generic over `Into`.
    record.set_title("no conversion at the call site");
    println!("title:   {}", record.title::<String>());

    // Every one of them also has a builder, with a body the macro wrote.
    let built = new_record().with_count(1).with_label(String::from("built"));
    println!();
    println!("built:   count {} label {:?}", built.count(), built.label());
}
