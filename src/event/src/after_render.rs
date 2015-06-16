use std::any::Any;

use GenericEvent;
use AfterRenderArgs;
use AFTER_RENDER;

/// After rendering and buffers are swapped.
pub trait AfterRenderEvent {
    /// Creates an after render event.
    fn from_after_render_args(args: &AfterRenderArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is an after render event.
    fn after_render<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U;
    /// Returns after render arguments.
    fn after_render_args(&self) -> Option<AfterRenderArgs> {
        self.after_render(|args| args.clone())
    }
}

impl<T: GenericEvent> AfterRenderEvent for T {
    fn from_after_render_args(args: &AfterRenderArgs, old_event: &T) -> Option<Self> {
        GenericEvent::from_args(AFTER_RENDER, args as &Any, old_event)
    }

    fn after_render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U
    {
        if self.event_id() != AFTER_RENDER {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<AfterRenderArgs>() {
                Some(f(args))
            } else {
                panic!("Expected AfterRenderArgs")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_after_render() {
        use Event;
        use AfterRenderArgs;

        let e = Event::AfterRender(AfterRenderArgs);
        let x: Option<Event> = AfterRenderEvent::from_after_render_args(
            &AfterRenderArgs, &e);
        let y: Option<Event> = x.clone().unwrap().after_render(|args|
            AfterRenderEvent::from_after_render_args(args, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
