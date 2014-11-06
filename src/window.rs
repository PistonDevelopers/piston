//! Game window operations.

use input::InputEvent;
use current::Get;

use GenericEvent;

/// Whether window should close or not.
pub struct ShouldClose(pub bool);

impl<T: Window<E>, E: GenericEvent>
Get<ShouldClose> for T {
    fn get(&self) -> ShouldClose {
        ShouldClose(self.should_close())
    }
}

/// The size of the window.
pub struct Size(pub [u32, ..2]);

impl<T: Window<E>, E: GenericEvent>
Get<Size> for T {
    fn get(&self) -> Size {
        let (w, h) = self.get_size();
        Size([w, h])
    }
}

/// Implemented by windows that can swap buffers.
pub trait SwapBuffers {
    /// Swaps the buffers.
    fn swap_buffers(&mut self);
}

/// Implemented by windows that can pull events.
pub trait PollEvent<E: GenericEvent> {
    /// Polls event from window.
    fn poll_event(&mut self) -> Option<E>;
}

/// Settings for window behavior.
pub struct WindowSettings {
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

impl WindowSettings {
    /// Gets default settings.
    ///
    /// This exits the window when pressing `Esc`.
    /// The background color is set to black.
    pub fn default() -> WindowSettings {
        WindowSettings {
            title: "Piston".to_string(),
            size: [640, 480],
            samples: 0,
            fullscreen: false,
            exit_on_esc: true,
        }
    }
}


/// Implemented by window back-end.
pub trait Window<E: GenericEvent = InputEvent>:
    SwapBuffers
  + PollEvent<E> {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a WindowSettings;

    /// Returns true if the window should close.
    fn should_close(&self) -> bool;

    /// Inform the window that it should close.
    fn close(&mut self);

    /// Get the window's size
    fn get_size(&self) -> (u32, u32);

    /// Get the size in drawing coordinates.
    fn get_draw_size(&self) -> (u32, u32);

    /// When the cursor is captured,
    /// it is hidden and the cursor position does not change.
    /// Only relative mouse motion is registered.
    fn capture_cursor(&mut self, _enabled: bool);
}

/// An implementation of GameWindow that represents running without a window at all
pub struct NoWindow {
    settings: WindowSettings,
    should_close: bool
}

impl NoWindow {
    /// Create a new nonexistant game window
    pub fn new(settings: WindowSettings) -> NoWindow {
         NoWindow {
             settings: settings,
             should_close: false
         }
    }
}

impl SwapBuffers for NoWindow {
    fn swap_buffers(&mut self) {}
}

impl PollEvent<InputEvent> for NoWindow {
    fn poll_event(&mut self) -> Option<InputEvent> { None }
}

impl Window<InputEvent> for NoWindow {
     fn get_settings<'a>(&'a self) -> &'a WindowSettings {
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

    fn get_draw_size(&self) -> (u32, u32) {
        self.get_size()
    }

    fn capture_cursor(&mut self, _enabled: bool) {}
}
