use std::any::{ Any, TypeId };

use GenericEvent;
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
    fn from_render_args(args: &RenderArgs) -> Option<Self> {
        let id = TypeId::of::<Box<RenderEvent>>();
        GenericEvent::from_args(id, args as &Any)
    }

    fn render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&RenderArgs) -> U
    {
        let id = TypeId::of::<Box<RenderEvent>>();
        if self.event_id() != id {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<RenderArgs>() {
                Some(f(args))
            } else {
                panic!("Expected RenderArgs")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_event_render() {
        use Event;
        use RenderArgs;

        let x: Option<Event> = RenderEvent::from_render_args(
            &RenderArgs {
                ext_dt: 1.0,
                width: 10,
                height: 10,
            }
        );
        let y: Option<Event> = x.clone().unwrap().render(|args|
            RenderEvent::from_render_args(args)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_render(bencher: &mut Bencher) {
        use Event;
        use RenderArgs;

        let args = RenderArgs {
            ext_dt: 1.0,
            width: 10,
            height: 10,
        };
        bencher.iter(|| {
            let _: Option<Event> = RenderEvent::from_render_args(&args);
        });
    }
}
