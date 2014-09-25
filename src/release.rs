use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };
use input::Button;

use GenericEvent;

/// Implemented by event structures that support release event.
pub trait ReleaseEvent {
    /// Creates a release event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a release event.
    fn press(&self, f: |Button|);
}

impl<T: GenericEvent> ReleaseEvent for T {
    #[inline(always)]
    fn from_button(button: Button) -> Option<T> {
        let id = TypeId::of::<Box<ReleaseEvent>>();
        GenericEvent::from_event(id, &button as &Any)
    }
    
    #[inline(always)]
    fn press(&self, f: |Button|) {
        let id = TypeId::of::<Box<ReleaseEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<Button>() {
                Some(&button) => f(button),
                None => fail!("Expected `Button`")
            }
        });
    }
}
