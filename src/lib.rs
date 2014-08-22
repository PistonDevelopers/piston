//! An experimental library using context types for event logic

#![crate_type = "lib"]
#![crate_id = "event#event:0.1"]
#![deny(missing_doc)]

extern crate collections;
extern crate time;
extern crate piston;

pub use event_type::EventType as EventType;
pub use event_type::KeyType as KeyType;

pub use add_after::AddAfter as AddAfter;
pub use add_all::AddAll as AddAll;
pub use add_any::AddAny as AddAny;
pub use add_interval::AddInterval as AddInterval;
pub use add_press::AddPress as AddPress;
pub use add_release::AddRelease as AddRelease;

pub use after_event::AfterEvent as AfterEvent;
pub use all_event::AllEvent as AllEvent;
pub use any_event::AnyEvent as AnyEvent;
pub use event::Event as Event;
pub use interval_event::IntervalEvent as IntervalEvent;
pub use press_event::PressEvent as PressEvent;
pub use release_press_event::ReleasePressEvent as ReleasePressEvent;

pub use call::Call as Call;
pub use call_once::CallOnce as CallOnce;
pub use triggered::Triggered as Triggered;

pub use event_center::EventCenter as EventCenter;
pub use observer::Observer as Observer;

mod event_type;

mod add_after;
mod add_all;
mod add_any;
mod add_press;
mod add_release;
mod add_interval;

mod after_event;
mod all_event;
mod any_event;
mod event;
mod interval_event;
mod press_event;
mod release_press_event;

mod call;
mod call_once;
mod triggered;

mod event_center;
mod observer;

/// ***************************
/// * COPY FROM RUST-GRAPHICS *
/// ***************************
///
/// A structure that might contain a value or a borrowed value.
/// This is to used as building block to create data structure
/// that is partially based on an existing structure.
pub enum Field<'a, T> {
    /// Contains a value.
    Value(T),
    /// Contains a borrowed pointer.
    Borrowed(&'a T),
}

impl<'a, T> Field<'a, T> {
    /// Gets a read only value.
    #[inline(always)]
    pub fn get(&'a self) -> &'a T {
        match *self {
            Value(ref val) => val,
            Borrowed(rval) => rval,
        }
    }
}

