//! An experimental library using expression for event logic
//!
//! The idea is to use combinators of events to describe more complex events.
//! An 'Action' is a variant of an event that spans across time.
//! You program the actions like a state machine, controlling how they interact with the world.
//!
//! Assume you have a complete list of the actions.
//! Any event you can construct from these actions has a corresponding `Cursor`.
//! The cursor keeps track of the combinatorial state.
//!
//! This design is useful in environments where all actions can be broken down
//! into simple interacitons while needing complex combinations of those actions.

#![crate_type = "lib"]
#![crate_id = "event#event:0.1"]
#![deny(missing_doc)]

pub use event::{
    Action,
    Event,
    Sequence,
    Wait,
    WhenAll,
    While,
};
pub use cursor::{
    Cursor,
    SequenceCursor,
    State,
    WaitCursor,
    WhenAllCursor,
    WhileCursor,
};
pub use start_state::StartState;

mod cursor;
mod event;
mod start_state;

