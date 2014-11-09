use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// Render arguments
#[deriving(Clone, PartialEq, Show)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area.
    pub width: u32,
    /// The height of rendered area.
    pub height: u32,
}

/// When the next frame should be rendered
pub trait RenderEvent {
    /// Creates a render event.
    fn from_render_args(args: &RenderArgs) -> Option<Self>;
    /// Calls closure if this is a render event.
    fn render<U>(&self, f: |&RenderArgs| -> U) -> Option<U>;
}

impl<T: GenericEvent> RenderEvent for T {
    #[inline(always)]
    fn from_render_args(args: &RenderArgs) -> Option<T> {
        let id = TypeId::of::<Box<RenderEvent>>();
        Ptr::with_ref::<RenderArgs, Option<T>>(args, |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }
    #[inline(always)]
    fn render<U>(&self, f: |&RenderArgs| -> U) -> Option<U> {
        let id = TypeId::of::<Box<RenderEvent>>();
        self.with_event(id, |ptr| {
            f(ptr.expect::<RenderArgs>())
        })
    }
}
