use viewport::Viewport;

use crate::{Event, Input};

/// Resize arguments.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct ResizeArgs {
    /// The width and height of rendered area in points.
    pub window_size: [f64; 2],
    /// The width of rendered area in pixels.
    pub draw_size: [u32; 2],
}

impl ResizeArgs {
    /// Returns viewport information filling entire render area.
    pub fn viewport(&self) -> Viewport {
        Viewport {
            rect: [0, 0, self.draw_size[0] as i32, self.draw_size[1] as i32],
            window_size: self.window_size,
            draw_size: self.draw_size,
        }
    }
}

/// When the window is resized.
pub trait ResizeEvent: Sized {
    /// Creates a resize event.
    ///
    /// Preserves time stamp from original input event, if any.
    fn from_resize_args(args: &ResizeArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut(&ResizeArgs) -> U;
    /// Returns resize arguments.
    fn resize_args(&self) -> Option<ResizeArgs> {
        self.resize(|args| *args)
    }
}

impl ResizeEvent for Event {
    fn from_resize_args(args: &ResizeArgs, old_event: &Self) -> Option<Self> {
        let timestamp = if let Event::Input(_, x) = old_event {
            *x
        } else {
            None
        };
        Some(Event::Input(Input::Resize(*args), timestamp))
    }

    fn resize<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut(&ResizeArgs) -> U,
    {
        match *self {
            Event::Input(Input::Resize(ref args), _) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_resize() {
        use super::super::Input;

        let args = ResizeArgs {
            window_size: [100.0, 100.0],
            draw_size: [100, 100],
        };
        let e: Event = Input::Resize(args).into();
        let x: Option<Event> = ResizeEvent::from_resize_args(&args, &e);
        let y: Option<Event> = x
            .clone()
            .unwrap()
            .resize(|args| ResizeEvent::from_resize_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
