use viewport::Viewport;

use Input;

/// Render arguments
#[derive(Copy, Clone, PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub struct RenderArgs {
    /// Extrapolated time in nanoseconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area in points.
    pub width: u32,
    /// The height of rendered area in points.
    pub height: u32,
    /// The width of rendered area in pixels.
    pub draw_width: u32,
    /// The height of rendered area in pixels.
    pub draw_height: u32,
}

impl RenderArgs {
    /// Returns viewport information filling entire render area.
    pub fn viewport(&self) -> Viewport {
        Viewport {
            rect: [0, 0, self.draw_width as i32, self.draw_height as i32],
            window_size: [self.width, self.height],
            draw_size: [self.draw_width, self.draw_height],
        }
    }
}

/// When the next frame should be rendered
pub trait RenderEvent: Sized {
    /// Creates a render event.
    fn from_render_args(args: &RenderArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a render event.
    fn render<U, F>(&self, f: F) -> Option<U> where F: FnMut(&RenderArgs) -> U;
    /// Returns render arguments.
    fn render_args(&self) -> Option<RenderArgs> {
        self.render(|args| args.clone())
    }
}

impl RenderEvent for Input {
    fn from_render_args(args: &RenderArgs, _old_event: &Self) -> Option<Self> {
        Some(Input::Render(*args))
    }

    fn render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&RenderArgs) -> U
    {
        match *self {
            Input::Render(ref args) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_render() {
        use Input;
        use RenderArgs;

        let e = Input::Render(RenderArgs {
            ext_dt: 0.0,
            width: 0,
            height: 0,
            draw_width: 0,
            draw_height: 0,
        });
        let x: Option<Input> = RenderEvent::from_render_args(&RenderArgs {
                                                                 ext_dt: 1.0,
                                                                 width: 10,
                                                                 height: 10,
                                                                 draw_width: 10,
                                                                 draw_height: 10,
                                                             },
                                                             &e);
        let y: Option<Input> = x.clone()
            .unwrap()
            .render(|args| RenderEvent::from_render_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
