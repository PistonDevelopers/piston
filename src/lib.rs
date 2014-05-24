//! An experimental library using context types for event logic

#![crate_type = "lib"]
#![crate_id = "event#event:0.1"]
//#![deny(missing_doc)]

extern crate collections;
extern crate graphics;


pub use EventType = event_type::EventType;
pub use KeyType = event_type::KeyType;

pub use AddInterval = add_interval::AddInterval;
pub use AddPress = add_press::AddPress;
pub use AddRelease = add_release::AddRelease;

pub use Event = event::Event;
pub use IntervalEvent = interval_event::IntervalEvent;
pub use PressEvent = press_event::PressEvent;
pub use ReleasePressEvent = release_press_event::ReleasePressEvent;

pub use Call = call::Call;
pub use Triggered = triggered::Triggered;

pub use EventCenter = event_center::EventCenter;
pub use Observer = observer::Observer;

mod event_type;
mod piston_event_type;

mod add_press;
mod add_release;
mod add_interval;

mod event;
mod interval_event;
mod press_event;
mod release_press_event;

mod call;
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

