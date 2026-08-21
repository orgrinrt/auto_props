//! Every feature selection this crate claims to support, and what each one does to a
//! consumer.
//!
//! The build checks are the cheap half. The half that matters is compiling a consumer
//! crate: `impl_with` and `getter_prefix` change what `property!` writes, and `no_std`
//! claims that what it writes works without `std`, and none of those can be seen from
//! inside this crate, because this crate is not the one the macro expands into.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Builds this crate under one feature selection.
fn check(features: &str) -> (bool, String) {
    let mut command = Command::new(env!("CARGO"));
    command
        .args(["check", "--quiet", "--no-default-features"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"),
        );
    if !features.is_empty() {
        command.args(["--features", features]);
    }
    let output = command.output().expect("cargo runs");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

/// Builds a throwaway crate whose body is `body`, against this crate at `features`.
fn consumer_compiles(name: &str, features: &str, attrs: &str, body: &str) -> (bool, String) {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers")).join(name);
    fs::create_dir_all(root.join("src")).expect("the consumer directory");

    let features_list = if features.is_empty() {
        String::new()
    } else {
        features
            .split(',')
            .map(|f| format!("\"{f}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };

    fs::write(
        root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2021"

[dependencies.auto_props]
path = "{crate_dir}"
default-features = false
features = [{features_list}]

[workspace]
"#,
            crate_dir = env!("CARGO_MANIFEST_DIR"),
        ),
    )
    .expect("the consumer manifest");

    fs::write(
        root.join("src").join("lib.rs"),
        format!("{attrs}\nuse auto_props::property;\n\n{body}\n"),
    )
    .expect("the consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["check", "--quiet"])
        .current_dir(&root)
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers/target"),
        )
        .output()
        .expect("cargo runs");

    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn every_selection_builds() {
    for selection in [
        "",
        "impl_with",
        "getter_prefix",
        "impl_with,getter_prefix",
        "no_std",
        "no_alloc",
        "no_alloc,no_std",
        "no_alloc,impl_with,getter_prefix",
    ] {
        let (ok, err) = check(selection);
        let label = if selection.is_empty() {
            "no features"
        } else {
            selection
        };
        assert!(ok, "{label} builds:\n{err}");
    }
}

/// A trait declared with every form of `property!`, for a consumer to compile.
const EVERY_FORM: &str = r#"
pub trait Shape {
    property!(width: u32);
    property!(height: u32 = _ -> _);
    property!(depth: u32 as Option<u32>);
}
"#;

#[test]
fn a_no_std_consumer_can_declare_a_trait_with_it() {
    // The thing the `no_std` feature exists for, and the only place it can be observed:
    // what `property!` writes lands in the consumer's crate, not in this one.
    let (ok, err) = consumer_compiles(
        "no_std_consumer",
        "no_alloc,impl_with",
        "#![no_std]",
        EVERY_FORM,
    );
    assert!(ok, "a `#![no_std]` consumer can declare the trait:\n{err}");
}

#[test]
fn that_consumer_really_is_without_std() {
    // The control. Without it the test above would pass just as well against a consumer
    // where `#![no_std]` did nothing at all.
    let (ok, err) = consumer_compiles(
        "no_std_control",
        "no_alloc",
        "#![no_std]",
        "pub fn reaches() { let _ = std::vec::Vec::<u8>::new(); }",
    );
    assert!(!ok, "a `#![no_std]` consumer must not reach `std::vec`");
    assert!(
        err.contains("std"),
        "the error is about `std` being absent:\n{err}"
    );
}

#[test]
fn impl_with_decides_whether_the_builder_is_written() {
    // With the flag, `with_width` exists and arrives with a body, so an implementor that
    // supplies only the getter and setter is complete.
    let implementor = r#"
pub trait Sized2 {
    property!(width: u32);
}

pub struct Boxy { w: u32 }

impl Sized2 for Boxy {
    fn width(&self) -> u32 { self.w }
    fn set_width(&mut self, value: u32) { self.w = value; }
}

pub fn build() -> Boxy { Boxy { w: 0 }.with_width(3) }
"#;

    let (ok, err) = consumer_compiles("with_present", "impl_with", "", implementor);
    assert!(ok, "`with_width` exists when `impl_with` is on:\n{err}");

    // Without it, the same call has nothing to resolve to. This is the direction a build
    // check cannot reach: a feature that gates nothing passes every positive test.
    let (ok, err) = consumer_compiles("with_absent", "", "", implementor);
    assert!(!ok, "`with_width` is absent when `impl_with` is off");
    assert!(err.contains("with_width"), "the error names it:\n{err}");
}

#[test]
fn getter_prefix_decides_what_the_getter_is_called() {
    let unprefixed = r#"
pub trait Named { property!(name: u32); }
pub fn read<N: Named>(n: &N) -> u32 { n.name() }
"#;
    let prefixed = r#"
pub trait Named { property!(name: u32); }
pub fn read<N: Named>(n: &N) -> u32 { n.get_name() }
"#;

    let (ok, err) = consumer_compiles("getter_plain", "", "", unprefixed);
    assert!(ok, "the getter is `name` without the flag:\n{err}");

    let (ok, err) = consumer_compiles("getter_prefixed", "getter_prefix", "", prefixed);
    assert!(ok, "the getter is `get_name` with the flag:\n{err}");

    // And each is absent in the other configuration, which is what makes the flag a
    // selection rather than an addition.
    let (ok, _) = consumer_compiles("getter_plain_absent", "getter_prefix", "", unprefixed);
    assert!(!ok, "`name` must not exist when the prefix is on");

    let (ok, _) = consumer_compiles("getter_prefixed_absent", "", "", prefixed);
    assert!(!ok, "`get_name` must not exist when the prefix is off");
}

#[test]
fn the_crate_depends_on_its_own_proc_macros_and_nothing_else() {
    // `paste` was a dependency that nothing in the crate used. Pinned, because an unused
    // dependency is invisible: it breaks no build and shows up only in what a consumer has
    // to compile.
    let manifest =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).expect("manifest");

    let deps = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("a dependencies section")
        .split("\n[")
        .next()
        .expect("the section body");

    let named: Vec<&str> = deps
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.split(['=', ' ']).next().unwrap_or(line))
        .collect();

    assert_eq!(
        named,
        ["auto_props_proc_macros"],
        "the dependency set has changed"
    );
}
