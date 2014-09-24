use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Render argument.
#[deriving(Clone, PartialEq, Show)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area.
    pub width: u32,
    /// The height of rendered area.
    pub height: u32,
}

/// Implemented by event structures that supports render event.
pub trait RenderEvent {
    /// Creates a render event.
    fn from_render_args(args: &RenderArgs) -> Option<Self>;
    /// Calls closure if this is a render event.
    fn update(&self, f: |&RenderArgs|);
}

impl<T: GenericEvent> RenderEvent for T {
    #[inline(always)]
    fn from_render_args(args: &RenderArgs) -> Option<T> {
        let id = TypeId::of::<Box<RenderEvent>>();
        GenericEvent::from_event(id, args as &Any)
    }
    #[inline(always)]
    fn update(&self, f: |&RenderArgs|) {
        let id = TypeId::of::<Box<RenderEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<RenderArgs>() {
                Some(args) => f(args),
                None => fail!("Expected `RenderArgs`")
            }
        });
    }
}
