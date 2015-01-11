use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;
use RenderArgs;

/// When the next frame should be rendered
pub trait RenderEvent {
    /// Creates a render event.
    fn from_render_args(args: &RenderArgs) -> Option<Self>;
    /// Calls closure if this is a render event.
    fn render<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&RenderArgs) -> U;
    /// Returns render arguments.
    fn render_args(&self) -> Option<RenderArgs> {
        self.render(|args| args.clone())
    }
}

impl<T: GenericEvent> RenderEvent for T {
    #[inline(always)]
    fn from_render_args(args: &RenderArgs) -> Option<T> {
        let id = TypeId::of::<Box<RenderEvent>>();
        Ptr::with_ref::<RenderArgs, Option<T>, _>(args, |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }
    #[inline(always)]
    fn render<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(&RenderArgs) -> U
    {
        let id = TypeId::of::<Box<RenderEvent>>();
        self.with_event(id, |&mut: ptr| {
            f(ptr.expect::<RenderArgs>())
        })
    }
}
