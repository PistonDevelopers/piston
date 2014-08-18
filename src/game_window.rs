//! Game window operations.
use mouse;
use keyboard;

/// Used by window back-end to model window events.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Event {
    /// No event occured.
    NoEvent,
    /// A key was released.
    KeyReleased(keyboard::Key),
    /// A key was pressed.
    KeyPressed(keyboard::Key),
    /// A mouse button was pressed.
    MouseButtonPressed(mouse::Button),
    /// A mouse button was released.
    MouseButtonReleased(mouse::Button),
    /// The mouse moved.
    ///
    /// (x, y, Some((dx, dy)))
    /// The second part is relative motion which is not bounded.
    /// Relative motion might not be supported by all window back-ends.
    MouseMoved(f64, f64, Option<(f64, f64)>),
    /// The mouse wheel.
    ///
    /// (x, y)
    MouseScrolled(f64, f64),
}

/// Settings for window behavior.
pub struct GameWindowSettings {
    /// Title of the window.
    pub title: String,
    /// The size of the window
    pub size: [u32, ..2],
    /// If true, the window is fullscreen.
    pub fullscreen: bool,
    /// If true, exit when pressing Esc.
    pub exit_on_esc: bool,
}

impl GameWindowSettings {
    /// Gets default settings.
    ///
    /// This exits the window when pressing `Esc`.
    /// The background color is set to black.
    pub fn default() -> GameWindowSettings {
        GameWindowSettings {
            title: "Piston".to_string(),
            size: [640, 480],
            fullscreen: false,
            exit_on_esc: true,
        }
    }
}


/// Implemented by window back-end.
pub trait GameWindow {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings;

    /// Returns ture if the window should close.
    fn should_close(&self) -> bool;

    /// Inform the window that it should close.
    fn close(&mut self);

    /// Get the window's size
    fn get_size(&self) -> (u32, u32) {
        (self.get_settings().size[0], self.get_settings().size[1])
    }

    /// Get the size in drawing coordinates.
    fn get_draw_size(&self) -> (u32, u32) {
        self.get_size()
    }

    /// Swap buffers.
    fn swap_buffers(&self) {}

    /// When the cursor is captured,
    /// it is hidden and the cursor position does not change.
    /// Only relative mouse motion is registered.
    fn capture_cursor(&mut self, _enabled: bool) {}
    
    /// Poll a event from window's event queue.
    fn poll_event(&mut self) -> Event { NoEvent }
}

