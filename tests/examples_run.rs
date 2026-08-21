//! Every example builds and runs, and prints something.
//!
//! `every_arm` is the load-bearing one: it writes each shape `property!` accepts and then
//! implements every accessor by hand, so an arm whose expansion changes shape stops the build
//! rather than quietly writing a different method. That is a check `tests/arms.rs` also makes,
//! and the difference is that an example is what a reader copies.
//!
//! The empty-output assertion catches an example whose `main` was left as a stub, which is the
//! one failure building cannot see.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// The examples, read from the directory rather than listed, so one added without being named
/// here still runs.
fn examples() -> Vec<String> {
    let dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/examples"));
    let mut names: Vec<String> = fs::read_dir(dir)
        .expect("examples/ exists")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "rs"))
        .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .collect();
    names.sort();
    names
}

#[test]
fn every_example_runs_and_says_something() {
    let names = examples();
    assert!(
        names.len() >= 2,
        "found {} examples, which is fewer than the directory is supposed to hold. An example \
         deleted stops being checked, and nothing else would report it.",
        names.len()
    );

    for name in &names {
        let output = Command::new(env!("CARGO"))
            .args(["run", "--quiet", "--example", name])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .env(
                "CARGO_TARGET_DIR",
                concat!(env!("CARGO_MANIFEST_DIR"), "/target/examples-run"),
            )
            .output()
            .expect("cargo runs");

        assert!(
            output.status.success(),
            "`cargo run --example {name}` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !output.stdout.is_empty(),
            "`{name}` ran and printed nothing, so whatever it was showing is not being shown"
        );
    }
}
