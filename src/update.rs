use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;
use UpdateArgs;

/// When the application state should be updated
pub trait UpdateEvent {
    /// Creates an update event.
    fn from_update_args(args: &UpdateArgs) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64) -> Option<Self>;
    /// Calls closure if this is an update event.
    fn update<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U;
    /// Returns update arguments.
    fn update_args(&self) -> Option<UpdateArgs> {
        self.update(|args| args.clone())
    }
}

impl<T: GenericEvent> UpdateEvent for T {
    #[inline(always)]
    fn from_update_args(args: &UpdateArgs) -> Option<T> {
        let id = TypeId::of::<Box<UpdateEvent>>();
        Ptr::with_ref::<UpdateArgs, Option<T>, _>(args, |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn from_dt(dt: f64) -> Option<T> {
        UpdateEvent::from_update_args(&UpdateArgs { dt: dt })
    }

    #[inline(always)]
    fn update<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(&UpdateArgs) -> U
    {
        let id = TypeId::of::<Box<UpdateEvent>>();
        self.with_event(id, |&mut: ptr| {
            f(ptr.expect::<UpdateArgs>())
        })
    }
}
