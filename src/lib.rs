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
pub use AddRelease = add_release::AddRelease;

pub use Event = event::Event;
pub use KeyboardEvent = keyboard_event::KeyboardEvent;
pub use PressKeyboardEvent = press_keyboard_event::PressKeyboardEvent;
pub use PressingKeyboardEvent = pressing_keyboard_event::PressingKeyboardEvent;
pub use LastingPressingKeyboardEvent = lasting_pressing_keyboard_event::LastingPressingKeyboardEvent;
pub use ReleaseKeyboardEvent = release_keyboard_event::ReleaseKeyboardEvent;

pub use Call = call::Call;

pub use BackEnd = back_end::BackEnd;
pub use Observer = observer::Observer;

mod add_keyboard;
mod add_lasting;
mod add_press;
mod add_pressing;
mod add_release;

mod event;
mod keyboard_event;
mod press_keyboard_event;
mod pressing_keyboard_event;
mod lasting_pressing_keyboard_event;
mod release_keyboard_event;

mod call;

mod back_end;
mod observer;

