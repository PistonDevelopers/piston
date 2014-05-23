//! An experimental library using context types for event logic

#![crate_type = "lib"]
#![crate_id = "event#event:0.1"]
//#![deny(missing_doc)]

extern crate collections;
extern crate graphics;

pub use AddPress = add_press::AddPress;

pub use Event = event::Event;
pub use EventType = event_type::EventType;
pub use KeyType = event_type::KeyType;
pub use PressEvent = press_event::PressEvent;

pub use Call = call::Call;

pub use EventCenter = event_center::EventCenter;
pub use Observer = observer::Observer;

mod add_press;

mod event;
mod event_type;
mod press_event;

mod call;

mod event_center;
mod piston_event_type;
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

