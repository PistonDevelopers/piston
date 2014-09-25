use std::intrinsics::{ get_tydesc, TypeId };
use std::any::{ Any, AnyRefExt };
use std::fmt::Show;
use input::{ Button, InputEvent, Press, Release };

use {
    PressEvent,
    ReleaseEvent,
};

/// Used as generic constraint for events.
///
/// Methods should not be called directly.
///
/// An `event_trait_id` is created from `TypeId::of::<Box<Trait>>()`.
/// The implementation need to know the argument type for all event traits.
/// Implementations of `GenericEvent` should be unit tested.
pub trait GenericEvent {
    /// Creates a new event.
    fn from_event(event_trait_id: TypeId, args: &Any) -> Option<Self>;
    /// When correct event type, calls closure with argument.
    fn with_event(&self, event_trait_id: TypeId, f: |&Any|);
}

/// Asserts that an event is supported correctly and is that event.
///
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
        fail!("Expected event trait '{}', found '{}'", name, e);
    }
}

impl GenericEvent for InputEvent {
    #[inline(always)]
    fn from_event(event_trait_id: TypeId, args: &Any) -> Option<InputEvent> {
        let press = TypeId::of::<Box<PressEvent>>();
        let release = TypeId::of::<Box<ReleaseEvent>>();
        match event_trait_id {
            x if x == press => {
                match args.downcast_ref::<Button>() {
                    Some(&button) => Some(Press(button)),
                    _ => fail!("Expected `Button`")
                }
            }
            x if x == release => {
                match args.downcast_ref::<Button>() {
                    Some(&button) => Some(Release(button)),
                    _ => fail!("Expected `Button`")
                }
            }
            _ => None
        }
    }

    #[inline(always)]
    fn with_event(&self, event_trait_id: TypeId, f: |&Any|) {
        let press = TypeId::of::<Box<PressEvent>>();
        let release = TypeId::of::<Box<ReleaseEvent>>();
        match event_trait_id {
            x if x == press => {
                match *self {
                    Press(ref button) => f(button),
                    _ => {}
                }
            }
            x if x == release => {
                match *self {
                    Release(ref button) => f(button),
                    _ => {}
                }
            }
            _ => {}
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
}
