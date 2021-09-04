use viewport::Viewport;

use crate::{Event, Loop};

/// Render arguments.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width and height of rendered area in points.
    pub window_size: [f64; 2],
    /// The width and height of rendered area in pixels.
    pub draw_size: [u32; 2],
}

impl RenderArgs {
    /// Returns viewport information filling entire render area.
    pub fn viewport(&self) -> Viewport {
        Viewport {
            rect: [0, 0, self.draw_size[0] as i32, self.draw_size[1] as i32],
            window_size: self.window_size,
            draw_size: self.draw_size,
        }
    }
}

/// When the next frame should be rendered.
pub trait RenderEvent: Sized {
    /// Creates a render event.
    fn from_render_args(args: &RenderArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a render event.
    fn render<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut(&RenderArgs) -> U;
    /// Returns render arguments.
    fn render_args(&self) -> Option<RenderArgs> {
        self.render(|args| *args)
    }
}

impl RenderEvent for Event {
    fn from_render_args(args: &RenderArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Loop(Loop::Render(*args)))
    }

    fn render<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut(&RenderArgs) -> U,
    {
        match *self {
            Event::Loop(Loop::Render(ref args)) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_render() {
        use RenderArgs;

        let e: Event = RenderArgs {
            ext_dt: 0.0,
            window_size: [0.0, 0.0],
            draw_size: [0, 0],
        }
        .into();
        let x: Option<Event> = RenderEvent::from_render_args(
            &RenderArgs {
                ext_dt: 1.0,
                window_size: [10.0, 10.0],
                draw_size: [10, 10],
            },
            &e,
        );
        let y: Option<Event> = x
            .clone()
            .unwrap()
            .render(|args| RenderEvent::from_render_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
