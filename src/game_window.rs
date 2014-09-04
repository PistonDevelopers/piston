//! Game window operations.

use input::InputEvent;

/// Settings for window behavior.
pub struct GameWindowSettings {
    /// Title of the window.
    pub title: String,
    /// The size of the window.
    pub size: [u32, ..2],
    /// Number samples per pixel (anti-aliasing).
    pub samples: u8,
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
            samples: 0,
            fullscreen: false,
            exit_on_esc: true,
        }
    }
}


/// Implemented by window back-end.
pub trait GameWindow {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings;

    /// Returns true if the window should close.
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
    fn poll_event(&mut self) -> Option<InputEvent> { None }
}

/// An implementation of GameWindow that represents running without a window at all
pub struct NoGameWindow {
    settings: GameWindowSettings,
    should_close: bool
}

impl NoGameWindow {
    /// Create a new nonexistant game window
    pub fn new(settings: GameWindowSettings) -> NoGameWindow {
         NoGameWindow {
             settings: settings,
             should_close: false
         }
    }
}

impl GameWindow for NoGameWindow {
     fn get_settings<'a>(&'a self) -> &'a GameWindowSettings {
        &self.settings
     }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn close(&mut self) {
        self.should_close = true
    }

    fn get_size(&self) -> (u32, u32) {
        (0, 0)
    }
}
