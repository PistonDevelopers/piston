//! An experimental library using context types for event logic

#![crate_type = "lib"]
#![crate_id = "event#event:0.1"]
#![deny(missing_doc)]

extern crate graphics;
extern crate piston;

pub use AddKeyboard = add_keyboard::AddKeyboard;
pub use AddLasting = add_lasting::AddLasting;
pub use AddPress = add_press::AddPress;
pub use AddPressing = add_pressing::AddPressing;

pub use Event = event::Event;
pub use KeyboardEvent = keyboard_event::KeyboardEvent;
pub use KeyboardPressingEvent = keyboard_pressing_event::KeyboardPressingEvent;
pub use KeyboardPressingLastingEvent = keyboard_pressing_lasting_event::KeyboardPressingLastingEvent;
pub use KeyboardPressEvent = keyboard_press_event::KeyboardPressEvent;

pub use Map = map::Map;

pub use BackEnd = back_end::BackEnd;
pub use Observer = observer::Observer;

mod add_keyboard;
mod add_lasting;
mod add_press;
mod add_pressing;

mod event;
mod keyboard_event;
mod keyboard_pressing_event;
mod keyboard_pressing_lasting_event;
mod keyboard_press_event;

mod map;

mod back_end;
mod observer;

