#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]
#![feature(macro_reexport)]

//! A user friendly game engine written in Rust.

// Reexported crates.
extern crate input as input_lib;
extern crate event as event_lib;
extern crate window as window_lib;
#[macro_reexport(quack, quack_get, quack_set, quack_action, quack_macro_items)]
extern crate quack as quack_lib;

pub use input_lib as input;
pub use event_lib as event;
pub use window_lib as window;
pub use quack_lib as quack;

pub use event::events;

pub use quack::{
    Action,
    ActOn,
    Get,
    GetFrom,
    Set,
    SetAt,
};

