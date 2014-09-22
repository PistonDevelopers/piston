//! A Behavior Tree implementation for event logic and game AI
//!
//! Each action returns either `Success`, `Failure` or `Running`.
//! Actions are combined with behaviors such as `Wait` and `Select`.
//! The combined behavior is stored in a `Behavior` object.
//!
//! For each `Behavior` there is a `State`.
//! The state tracks the behavior over time.

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_doc)]

extern crate time;
extern crate input;

pub use window::{
    Window,
    WindowSettings,
    NoWindow
};
pub use event_iterator::{
    Render,
    Update,
    Input,
    Event,
    EventIterator,
    EventSettings,
    RenderArgs,
    UpdateArgs,
};
pub use status::{
    Failure,
    Running,
    Status,
    Success,
};
pub use behavior::{
    Action,
    AlwaysSucceed,
    Behavior,
    If,
    Fail,
    Pressed,
    Released,
    Select,
    Sequence,
    Wait,
    WaitForever,
    WhenAll,
    WhenAny,
    While,
};
pub use state::{
    AlwaysSucceedState,
    ActionState,
    IfState,
    FailState,
    PressedState,
    ReleasedState,
    SelectState,
    SequenceState,
    State,
    WaitState,
    WaitForeverState,
    WhenAllState,
    WhenAnyState,
    WhileState,
};

mod state;
mod behavior;
mod status;
mod window;
mod event_iterator;
