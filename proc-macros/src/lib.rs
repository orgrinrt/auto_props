// `common.rs` sits at the crate root rather than under `src/`, and this declares the proc macro
// from it. The 1.x `include_proc_macro!` that used to do this was removed in 2.x, where the
// macro generates the entry point and includes the implementation file behind it.
include_proc_macro::proc_macro!(common -> @"common.rs"::common);
