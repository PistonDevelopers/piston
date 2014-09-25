use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };
use input::Button;

use GenericEvent;

/// Implemented by event structures that support press event.
pub trait PressEvent {
    /// Creates a press event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press(&self, f: |Button|);
}

impl<T: GenericEvent> PressEvent for T {
    #[inline(always)]
    fn from_button(button: Button) -> Option<T> {
        let id = TypeId::of::<Box<PressEvent>>();
        GenericEvent::from_event(id, &button as &Any)
    }
    
    #[inline(always)]
    fn press(&self, f: |Button|) {
        let id = TypeId::of::<Box<PressEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<Button>() {
                Some(&button) => f(button),
                None => fail!("Expected `Button`")
            }
        });
    }
}
