use Event;
use Input;

/// After render arguments.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AfterRenderArgs;

/// After rendering and buffers are swapped.
pub trait AfterRenderEvent: Sized {
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

/* TODO: Enable when specialization gets stable.
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
*/

impl AfterRenderEvent for Input {
    fn from_after_render_args(_args: &AfterRenderArgs, _old_event: &Input) -> Option<Self> {
        None
    }

    fn after_render<U, F>(&self, mut _f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U
    {
        None
    }
}

impl<I> AfterRenderEvent for Event<I> {
    fn from_after_render_args(args: &AfterRenderArgs, _old_event: &Event<I>) -> Option<Self> {
        Some(Event::AfterRender(*args))
    }

    fn after_render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U
    {
        match *self {
            Event::AfterRender(ref args) => Some(f(args)),
            _ => None
        }
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
