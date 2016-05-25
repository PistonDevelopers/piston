
//! An implementation of Window that runs without a window at all.
//!
//! It saves just enough information to implement the window interface,
//! but otherwise does nothing.

extern crate input;

use {
	Window,
	WindowSettings,
	BuildFromWindowSettings,
	AdvancedWindow
};

use input::Input;

/// The tiny NoWindow structure.
///
/// This structure holds just enough state to be able to return
/// values that were set.
pub struct NoWindow {
    should_close: bool,
    title: String,
    size: Size
}

impl NoWindow {
    /// Returns a new `NoWindow`.
    pub fn new(settings: WindowSettings) -> NoWindow {
        NoWindow {
            should_close: false,
            title: settings.title,
            size: settings.size,
        }
    }
}

impl Window for NoWindow {
	/// Uses the usual event type from the input module.
    type Event = Input;
    
    /// Gets the stored value for `should_close`.
    fn should_close(&self) -> bool { self.should_close }
    
    /// Saves a new value for `should_close`.
    ///
    /// Otherwise a no-op.
    fn set_should_close(&mut self, value: bool) { self.should_close = value; }
    
    /// Saves a new value for `size`.
    fn size(&self) -> Size { self.size }
    
    /// A no-op.
    fn swap_buffers(&mut self) {}
    
    /// A no-op. Always returns None.
    fn poll_event(&mut self) -> Option<Input> { None }
    
    /// Gets the stored value for `size`.
    fn draw_size(&self) -> Size { self.size() }
}

impl BuildFromWindowSettings for NoWindow {
	/// The normal implementation of this method.
	///
	/// # Errors
	///
	/// This function will always return without error.
    fn build_from_window_settings(settings: WindowSettings)
    -> Result<Self, String> {
        Ok(NoWindow::new(settings))
    }
}

impl AdvancedWindow for NoWindow {
	/// Gets a copy of the saved value for `title`.
    fn get_title(&self) -> String { self.title.clone() }
    
    /// Stores a new value for `title`.
    ///
    /// Otherwise a no-op.
    fn set_title(&mut self, value: String) { self.title = value; }
    
    /// No-op.
    ///
    /// Always returns false.
    fn get_exit_on_esc(&self) -> bool { false }
    
    /// No-op. It doesn't even store state.
    fn set_exit_on_esc(&mut self, _value: bool) {}
    
    /// No-op.
    fn set_capture_cursor(&mut self, _value: bool) {}
}