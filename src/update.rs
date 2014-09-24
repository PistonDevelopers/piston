use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Update argument.
#[deriving(Clone)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// Implemented by event structures that supports update event.
pub trait UpdateEvent {
    /// Creates an update event.
    fn from_update_args(args: &UpdateArgs) -> Option<Self>;
    /// Calls closure if this is an update event.
    fn update(&self, f: |&UpdateArgs|);
}

impl<T: GenericEvent> UpdateEvent for T {
    #[inline(always)]
    fn from_update_args(args: &UpdateArgs) -> Option<T> {
        let id = TypeId::of::<Box<UpdateEvent>>();
        GenericEvent::from_event(id, &args as &Any)
    }
    #[inline(always)]
    fn update(&self, f: |&UpdateArgs|) {
        let id = TypeId::of::<Box<UpdateEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<UpdateArgs>() {
                Some(args) => f(args),
                None => fail!("Expected `UpdateArgs`")
            }
        });
    }
}
