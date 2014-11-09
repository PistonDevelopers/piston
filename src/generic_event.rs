use std::intrinsics::{ get_tydesc, TypeId };
use std::fmt::Show;
use input::{
    Button,
    InputEvent,
    Focus,
    Move,
    MouseCursor,
    MouseRelative,
    MouseScroll,
    Press,
    Release,
    Resize,
    Text,
};

use {
    FocusEvent,
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    PressEvent,
    ReleaseEvent,
    ResizeEvent,
    TextEvent,
};
use ptr::Ptr;

/// Used as generic constraint for events
///
/// Methods should not be called directly.
///
/// An `event_trait_id` is created from `TypeId::of::<Box<Trait>>()`.
/// The implementation need to know the argument type for all event traits.
/// Implementations of `GenericEvent` should be unit tested.
pub trait GenericEvent {
    /// Creates a new event.
    fn from_event(event_trait_id: TypeId, args: &Ptr) -> Option<Self>;
    /// When correct event type, calls closure with argument.
    fn with_event<U>(&self, event_trait_id: TypeId, f: |&Ptr| -> U) -> Option<U>;
}

/// Asserts that an event is supported correctly
///
/// This is used in unit tests to check that an implementation of `GenericEvent`
/// is implemented correctly for a given event.
/// The event must implement `PartialEq` to use this test.
/// Example: `assert_event_trait::<Event, Box<Render>>(&e);`
pub fn assert_event_trait<
    E: GenericEvent + PartialEq + Show,
    T: 'static
>(e: &E) {
    let name = unsafe { (*get_tydesc::<T>()).name };
    let id = TypeId::of::<T>();

    let mut tested_equal = false;
    e.with_event(id, |ev| {
        let new_e: E = GenericEvent::from_event(id, ev).expect(
            format!(
                "Could not construct event of event trait '{}' from '{}'",
                name, e
            ).as_slice()
        );
        assert_eq!(*e, new_e);
        tested_equal = true;
    });
    if !tested_equal {
        panic!("Expected event trait '{}', found '{}'", name, e);
    }
}

impl GenericEvent for InputEvent {
    #[inline(always)]
    fn from_event(event_trait_id: TypeId, args: &Ptr) -> Option<InputEvent> {
        let press = TypeId::of::<Box<PressEvent>>();
        let release = TypeId::of::<Box<ReleaseEvent>>();
        let mouse_cursor = TypeId::of::<Box<MouseCursorEvent>>();
        let mouse_relative = TypeId::of::<Box<MouseRelativeEvent>>();
        let mouse_scroll = TypeId::of::<Box<MouseScrollEvent>>();
        let text = TypeId::of::<Box<TextEvent>>();
        let resize = TypeId::of::<Box<ResizeEvent>>();
        let focus = TypeId::of::<Box<FocusEvent>>();
        match event_trait_id {
            x if x == press => {
                Some(Press(*args.expect::<Button>()))
            }
            x if x == release => {
                Some(Release(*args.expect::<Button>()))
            }
            x if x == mouse_cursor => {
                let &(x, y) = args.expect::<(f64, f64)>();
                Some(Move(MouseCursor(x, y)))
            }
            x if x == mouse_relative => {
                let &(x, y) = args.expect::<(f64, f64)>();
                Some(Move(MouseRelative(x, y)))
            }
            x if x == mouse_scroll => {
                let &(x, y) = args.expect::<(f64, f64)>();
                Some(Move(MouseScroll(x, y)))
            }
            x if x == text => {
                let text = args.expect_str();
                Some(Text(text.to_string()))
            }
            x if x == resize => {
                let &(w, h) = args.expect::<(u32, u32)>();
                Some(Resize(w, h))
            }
            x if x == focus => {
                Some(Focus(*args.expect::<bool>()))
            }
            _ => None
        }
    }

    #[inline(always)]
    fn with_event<U>(&self, event_trait_id: TypeId, f: |&Ptr| -> U) -> Option<U> {
        let press = TypeId::of::<Box<PressEvent>>();
        let release = TypeId::of::<Box<ReleaseEvent>>();
        let mouse_cursor = TypeId::of::<Box<MouseCursorEvent>>();
        let mouse_relative = TypeId::of::<Box<MouseRelativeEvent>>();
        let mouse_scroll = TypeId::of::<Box<MouseScrollEvent>>();
        let text = TypeId::of::<Box<TextEvent>>();
        let resize = TypeId::of::<Box<ResizeEvent>>();
        let focus = TypeId::of::<Box<FocusEvent>>();
        match event_trait_id {
            x if x == press => {
                match *self {
                    Press(ref button) => Some(Ptr::with_ref(button, f)),
                    _ => None
                }
            }
            x if x == release => {
                match *self {
                    Release(ref button) => Some(Ptr::with_ref(button, f)),
                    _ => None
                }
            }
            x if x == mouse_cursor => {
                match *self {
                    Move(MouseCursor(x, y)) => Some(Ptr::with_ref(&(x, y), f)),
                    _ => None
                }
            }
            x if x == mouse_relative => {
                match *self {
                    Move(MouseRelative(x, y)) => Some(Ptr::with_ref(&(x, y), f)),
                    _ => None
                }
            }
            x if x == mouse_scroll => {
                match *self {
                    Move(MouseScroll(x, y)) => Some(Ptr::with_ref(&(x, y), f)),
                    _ => None
                }
            }
            x if x == text => {
                match *self {
                    Text(ref text) => Some(Ptr::with_str(text.as_slice(), f)),
                    _ => None
                }
            }
            x if x == resize => {
                match *self {
                    Resize(w, h) => Some(Ptr::with_ref(&(w, h), f)),
                    _ => None
                }
            }
            x if x == focus => {
                match *self {
                    Focus(focused) => Some(Ptr::with_ref(&focused, f)),
                    _ => None
                }
            }
            _ => None
        }
    }
}

#[test]
fn test_input_event() {
    use input;
    use input::Keyboard;

    let ref e = PressEvent::from_button(Keyboard(input::keyboard::A)).unwrap();
    assert_event_trait::<InputEvent, Box<PressEvent>>(e);

    let ref e = ReleaseEvent::from_button(Keyboard(input::keyboard::B)).unwrap();
    assert_event_trait::<InputEvent, Box<ReleaseEvent>>(e);

    let ref e = MouseCursorEvent::from_xy(1.0, 0.0).unwrap();
    assert_event_trait::<InputEvent, Box<MouseCursorEvent>>(e);

    let ref e = MouseRelativeEvent::from_xy(0.0, 1.0).unwrap();
    assert_event_trait::<InputEvent, Box<MouseRelativeEvent>>(e);

    let ref e = MouseScrollEvent::from_xy(-1.0, 0.0).unwrap();
    assert_event_trait::<InputEvent, Box<MouseScrollEvent>>(e);

    let ref e = TextEvent::from_text("hello").unwrap();
    assert_event_trait::<InputEvent, Box<TextEvent>>(e);

    let ref e = ResizeEvent::from_width_height(30, 33).unwrap();
    assert_event_trait::<InputEvent, Box<ResizeEvent>>(e);

    let ref e = FocusEvent::from_focused(true).unwrap();
    assert_event_trait::<InputEvent, Box<FocusEvent>>(e);
}
