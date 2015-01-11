use std::intrinsics::TypeId;
use input::Button;

use GenericEvent;
use ptr::Ptr;

/// The press of a button
pub trait PressEvent {
    /// Creates a press event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U;
    /// Returns press arguments.
    fn press_args(&self) -> Option<Button> {
        self.press(|button| button)
    }
}

impl<T: GenericEvent> PressEvent for T {
    #[inline(always)]
    fn from_button(button: Button) -> Option<T> {
        let id = TypeId::of::<Box<PressEvent>>();
        Ptr::with_ref::<Button, Option<T>, _>(&button, |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }
    
    #[inline(always)]
    fn press<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(Button) -> U
    {
        let id = TypeId::of::<Box<PressEvent>>();
        self.with_event(id, |&mut: ptr| {
            f(*ptr.expect::<Button>())
        })
    }
}
