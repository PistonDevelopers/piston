use std::intrinsics::TypeId;
use input::Button;

use GenericEvent;
use ptr::Ptr;

/// The press of a button
pub trait PressEvent {
    /// Creates a press event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U>(&self, f: |Button| -> U) -> Option<U>;
}

impl<T: GenericEvent> PressEvent for T {
    #[inline(always)]
    fn from_button(button: Button) -> Option<T> {
        let id = TypeId::of::<Box<PressEvent>>();
        Ptr::with_ref::<Button, Option<T>>(&button, |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }
    
    #[inline(always)]
    fn press<U>(&self, f: |Button| -> U) -> Option<U> {
        let id = TypeId::of::<Box<PressEvent>>();
        self.with_event(id, |ptr| {
            f(*ptr.expect::<Button>())
        })
    }
}
