use {Event, Loop};

/// After render arguments.
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AfterRenderArgs;

/// After rendering and buffers are swapped.
pub trait AfterRenderEvent: Sized {
    /// Creates an after render event.
    fn from_after_render_args(args: &AfterRenderArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is an after render event.
    fn after_render<U, F>(&self, f: F) -> Option<U> where F: FnMut(&AfterRenderArgs) -> U;
    /// Returns after render arguments.
    fn after_render_args(&self) -> Option<AfterRenderArgs> {
        self.after_render(|args| args.clone())
    }
}

impl AfterRenderEvent for Event {
    fn from_after_render_args(args: &AfterRenderArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Loop(Loop::AfterRender(*args)))
    }

    fn after_render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U
    {
        match *self {
            Event::Loop(Loop::AfterRender(ref args)) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_after_render() {
        use AfterRenderArgs;

        let e: Event = AfterRenderArgs.into();
        let x: Option<Event> = AfterRenderEvent::from_after_render_args(&AfterRenderArgs, &e);
        let y: Option<Event> =
            x.clone()
                .unwrap()
                .after_render(|args| {
                    AfterRenderEvent::from_after_render_args(args, x.as_ref().unwrap())
                })
                .unwrap();
        assert_eq!(x, y);
    }
}
