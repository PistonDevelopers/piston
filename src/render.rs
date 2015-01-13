use input::Input;

use Event;
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

impl RenderEvent for Input {
    fn from_render_args(_: &RenderArgs) -> Option<Self> {
        None
    }

    fn render<U, F>(&self, _: F) -> Option<U>
        where F: FnMut(&RenderArgs) -> U
    {
        None
    }
}

impl<I> RenderEvent for Event<I> {
    fn from_render_args(args: &RenderArgs) -> Option<Self> {
        Some(Event::Render(args.clone()))
    }

    fn render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&RenderArgs) -> U
    {
        if let &Event::Render(ref args) = self {
            Some(f(args))
        } else {
            None
        }
    }
}
