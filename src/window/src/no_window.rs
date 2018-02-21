
//! An implementation of Window that runs without a window at all.
//!
//! It saves just enough information to implement the window interface,
//! but otherwise does nothing.
//!
//! Often used in servers as an event loop.

use {Window, WindowSettings, BuildFromWindowSettings, AdvancedWindow, Position, Size};

use input::Input;
use std::time::Duration;

/// A window without user interface, often used in server event loops.
///
/// This structure holds just enough state to return values that were set.
/// The size can be changed because the event loop does not emit
/// [`Render`](../event_loop/trait.RenderEvent.html)
/// events when the width or height is zero.
pub struct NoWindow {
    should_close: bool,
    title: String,
    size: Size,
    pos: Position,
}

impl NoWindow {
    /// Creates a new `NoWindow`.
    pub fn new(settings: &WindowSettings) -> NoWindow {
        NoWindow {
            should_close: false,
            title: settings.get_title(),
            size: settings.get_size(),
            pos: Position { x: 0, y: 0 },
        }
    }
}

impl Window for NoWindow {
    fn should_close(&self) -> bool {
        self.should_close
    }

    fn set_should_close(&mut self, value: bool) {
        self.should_close = value;
    }

    fn size(&self) -> Size {
        self.size
    }

    fn swap_buffers(&mut self) {}

    fn wait_event(&mut self) -> Input {
        panic!("NoWindow will never return an input event");
    }

    fn wait_event_timeout(&mut self, _timeout: Duration) -> Option<Input> {
        None
    }

    fn poll_event(&mut self) -> Option<Input> {
        None
    }

    fn draw_size(&self) -> Size {
        self.size()
    }
}

impl BuildFromWindowSettings for NoWindow {
    /// # Errors
    ///
    /// This function will always return without error.
    fn build_from_window_settings(settings: &WindowSettings) -> Result<Self, String> {
        Ok(NoWindow::new(settings))
    }
}

impl AdvancedWindow for NoWindow {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn set_title(&mut self, value: String) {
        self.title = value;
    }

    fn get_exit_on_esc(&self) -> bool {
        false
    }

    fn set_exit_on_esc(&mut self, _value: bool) {}

    fn set_capture_cursor(&mut self, _value: bool) {}

    fn show(&mut self) {}

    fn hide(&mut self) {}

    fn get_position(&self) -> Option<Position> {
        Some(self.pos)
    }

    fn set_position<P: Into<Position>>(&mut self, val: P) {
        self.pos = val.into();
    }

    fn set_size<S: Into<Size>>(&mut self, val: S) {
        self.size = val.into();
    }
}
