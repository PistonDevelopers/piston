use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// Update arguments, such as delta time in seconds
#[deriving(Clone, PartialEq, Show)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// When the application state should be updated
pub trait UpdateEvent {
    /// Creates an update event.
    fn from_update_args(args: &UpdateArgs) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64) -> Option<Self>;
    /// Calls closure if this is an update event.
    fn update<U>(&self, f: |&UpdateArgs| -> U) -> Option<U>;
}

impl<T: GenericEvent> UpdateEvent for T {
    #[inline(always)]
    fn from_update_args(args: &UpdateArgs) -> Option<T> {
        let id = TypeId::of::<Box<UpdateEvent>>();
        Ptr::with_ref::<UpdateArgs, Option<T>>(args, |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn from_dt(dt: f64) -> Option<T> {
        UpdateEvent::from_update_args(&UpdateArgs { dt: dt })
    }

    #[inline(always)]
    fn update<U>(&self, f: |&UpdateArgs| -> U) -> Option<U> {
        let id = TypeId::of::<Box<UpdateEvent>>();
        self.with_event(id, |ptr| {
            f(ptr.expect::<UpdateArgs>())
        })
    }
}
