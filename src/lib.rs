#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]

//! A user friendly game engine written in Rust.

// Reexported crates.
pub extern crate input;
pub extern crate event;
pub extern crate window;
pub extern crate quack;

pub use event::{
    Event,
    Events,
    events,
    RenderArgs,
    UpdateArgs,
};

pub use quack::{
    Action,
    ActOn,
    Get,
    GetFrom,
    Set,
    SetAt,
};

