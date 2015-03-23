use std::any::{ Any, TypeId };

use GenericEvent;
use AfterRenderArgs;

/// After rendering and buffers are swapped.
pub trait AfterRenderEvent {
    /// Creates an after render event.
    fn from_after_render_args(args: &AfterRenderArgs) -> Option<Self>;
    /// Calls closure if this is an after render event.
    fn after_render<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U;
    /// Returns after render arguments.
    fn after_render_args(&self) -> Option<AfterRenderArgs> {
        self.after_render(|args| args.clone())
    }
}

impl<T: GenericEvent> AfterRenderEvent for T {
    fn from_after_render_args(args: &AfterRenderArgs) -> Option<Self> {
        let id = TypeId::of::<Box<AfterRenderEvent>>();
        GenericEvent::from_args(id, args as &Any)
    }

    fn after_render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U
    {
        let id = TypeId::of::<Box<AfterRenderEvent>>();
        if self.event_id() != id {
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
    use test::Bencher;

    #[test]
    fn test_event_after_render() {
        use Event;
        use AfterRenderArgs;

        let x: Option<Event> = AfterRenderEvent::from_after_render_args(
            &AfterRenderArgs
        );
        let y: Option<Event> = x.clone().unwrap().after_render(|args|
            AfterRenderEvent::from_after_render_args(args)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_after_render(bencher: &mut Bencher) {
        use Event;
        use AfterRenderArgs;

        let args = AfterRenderArgs;
        bencher.iter(|| {
            let _: Option<Event> = AfterRenderEvent::from_after_render_args(&args);
        });
    }
}
