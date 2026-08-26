#![crate_name = "piston"]
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all,
    clippy::doc_markdown,
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)]

#![doc = include_str!("../README.md")]

// Reexported crates.
pub use event_loop::{self, *};
pub use input::{self, *};
pub use window::{self, *};
