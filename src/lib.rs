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
#![feature(default_type_params)]

extern crate time;
extern crate input;

pub use window::{
    Window,
    WindowSettings,
    NoWindow
};
pub use event_iterator::{
    EventIterator,
    EventSettings,
};
pub use status::{
    Failure,
    Running,
    Status,
    Success,
};
pub use behavior::{
    Action,
    After,
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
    ActionState,
    AfterState,
    AlwaysSucceedState,
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
pub use generic_event::{ assert_event_trait, GenericEvent };
pub use update::{ UpdateArgs, UpdateEvent };
pub use render::{ RenderArgs, RenderEvent };
pub use event::{ Event, Render, Update, Input };
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use text::TextEvent;
pub use resize::ResizeEvent;
pub use focus::FocusEvent;

mod state;
mod behavior;
mod status;
mod window;
mod event_iterator;
mod generic_event;
mod update;
mod render;
mod event;
mod press;
mod release;
mod mouse;
mod text;
mod resize;
mod focus;
