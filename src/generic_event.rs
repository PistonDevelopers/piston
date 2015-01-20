//! Trait for generic events

use std::intrinsics::TypeId;
use std::borrow::ToOwned;
use std::any::Any;

use input::{ Button, Input, Motion };
use {
    Event,
    FocusEvent,
    PressEvent,
    ReleaseEvent,
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    TextEvent,
    ResizeEvent,
    UpdateEvent,
    RenderEvent,
    UpdateArgs,
    RenderArgs,
};

/// Implemented by all events
pub trait GenericEvent {
    /// The id of this event, usually `TypeOf::of::<Box<EventTrait>>().hash()`
    fn event_id(&self) -> u64;
    /// Calls closure with table
    fn with_args<'a, F, U>(&'a self, f: F) -> U
        where F: FnMut(&Any) -> U
    ;
    /// Converts from table to `Self`
    fn from_args(event_id: u64, any: &Any) -> Option<Self>;
}

impl GenericEvent for Input {
    fn event_id(&self) -> u64 {
        match self {
            &Input::Focus(_) => {
                TypeId::of::<Box<FocusEvent>>().hash()
            }
            &Input::Press(_) => {
                TypeId::of::<Box<PressEvent>>().hash()
            }
            &Input::Release(_) => {
                TypeId::of::<Box<ReleaseEvent>>().hash()
            }
            &Input::Move(Motion::MouseCursor(_, _)) => {
                TypeId::of::<Box<MouseCursorEvent>>().hash()
            }
            &Input::Move(Motion::MouseRelative(_, _)) => {
                TypeId::of::<Box<MouseRelativeEvent>>().hash()
            }
            &Input::Move(Motion::MouseScroll(_, _)) => {
                TypeId::of::<Box<MouseScrollEvent>>().hash()
            }
            &Input::Text(_) => {
                TypeId::of::<Box<TextEvent>>().hash()
            }
            &Input::Resize(_, _) => {
                TypeId::of::<Box<ResizeEvent>>().hash()
            }
        }
    }

    fn with_args<'a, F, U>(&'a self, mut f: F) -> U
        where F: FnMut(&Any) -> U
    {
        match self {
            &Input::Focus(focused) => {
                f(&focused as &Any)
            }
            &Input::Press(button) => {
                f(&button as &Any)
            }
            &Input::Release(button) => {
                f(&button as &Any)
            }
            &Input::Move(Motion::MouseCursor(x, y)) => {
                f(&(x, y) as &Any)
            }
            &Input::Move(Motion::MouseRelative(x, y)) => {
                f(&(x, y) as &Any)
            }
            &Input::Move(Motion::MouseScroll(x, y)) => {
                f(&(x, y) as &Any)
            }
            &Input::Text(ref text) => {
                f(text as &Any)
            }
            &Input::Resize(w, h) => {
                f(&(w, h) as &Any)
            }
        }
    }

    fn from_args(event_id: u64, any: &Any) -> Option<Self> {
        let focus_id = TypeId::of::<Box<FocusEvent>>().hash();
        let mouse_cursor_id = TypeId::of::<Box<MouseCursorEvent>>().hash();
        let mouse_relative_id = TypeId::of::<Box<MouseRelativeEvent>>().hash();
        let mouse_scroll_id = TypeId::of::<Box<MouseScrollEvent>>().hash();
        let press_id = TypeId::of::<Box<PressEvent>>().hash();
        let release_id = TypeId::of::<Box<ReleaseEvent>>().hash();
        let resize_id = TypeId::of::<Box<ResizeEvent>>().hash();
        let text_id = TypeId::of::<Box<TextEvent>>().hash();

        match event_id {
            x if x == focus_id => {
                if let Some(&focused) = any.downcast_ref::<bool>() {
                    Some(Input::Focus(focused))
                } else {
                    panic!("Expected bool")
                }
            }
            x if x == mouse_cursor_id => {
                if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                    Some(Input::Move(Motion::MouseCursor(x, y)))
                } else {
                    panic!("Expected (f64, f64)")
                }
            }
            x if x == mouse_relative_id => {
                if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                    Some(Input::Move(Motion::MouseRelative(x, y)))
                } else {
                    panic!("Expected (f64, f64)")
                }
            }
            x if x == mouse_scroll_id => {
                if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                    Some(Input::Move(Motion::MouseScroll(x, y)))
                } else {
                    panic!("Expected (f64, f64)")
                }
            }
            x if x == press_id => {
                if let Some(&button) = any.downcast_ref::<Button>() {
                    Some(Input::Press(button))
                } else {
                    panic!("Expected Button")
                }
            }
            x if x == release_id => {
                if let Some(&button) = any.downcast_ref::<Button>() {
                    Some(Input::Release(button))
                } else {
                    panic!("Expected Button")
                }
            }
            x if x == resize_id => {
                if let Some(&(w, h)) = any.downcast_ref::<(u32, u32)>() {
                    Some(Input::Resize(w as u32, h as u32))
                } else {
                    panic!("Expected (u32, u32))")
                }
            }
            x if x == text_id => {
                if let Some(text) = any.downcast_ref::<String>() {
                    Some(Input::Text(text.to_owned()))
                } else {
                    panic!("Expected &str")
                }
            }
            _ => { return None; }
        }
    }
}

impl<I: GenericEvent> GenericEvent for Event<I> {
    fn event_id(&self) -> u64 {
        match self {
            &Event::Update(_) => {
                TypeId::of::<Box<UpdateEvent>>().hash()
            }
            &Event::Render(_) => {
                TypeId::of::<Box<RenderEvent>>().hash()
            }
            &Event::Input(ref input) => {
                input.event_id()
            }
        }
    }

    fn with_args<'a, F, U>(&'a self, mut f: F) -> U
        where F: FnMut(&Any) -> U
    {
        match self {
            &Event::Update(ref args) => {
                f(args as &Any)
            }
            &Event::Render(ref args) => {
                f(args as &Any)
            }
            &Event::Input(ref input) => {
                input.with_args(f)
            }
        }
    }

    fn from_args(event_id: u64, any: &Any) -> Option<Self> {
        let update_id = TypeId::of::<Box<UpdateEvent>>().hash();
        let render_id = TypeId::of::<Box<RenderEvent>>().hash();

        match event_id {
            x if x == update_id => {
                if let Some(&args) = any.downcast_ref::<UpdateArgs>() {
                    Some(Event::Update(args))
                } else {
                    panic!("Expected UpdateArgs")
                }
            }
            x if x == render_id => {
                if let Some(&args) = any.downcast_ref::<RenderArgs>() {
                    Some(Event::Render(args))
                } else {
                    panic!("Expected RenderArgs")
                }
            }
            _ => {
                let input: Option<I> =
                    GenericEvent::from_args(event_id, any);
                match input {
                    Some(x) => Some(Event::Input(x)),
                    None => None
                }
            }
        }
    }
}
