//! A trait for window operations.

use event;
use game_window_settings::GameWindowSettings;

/// Implemented by window back-end.
pub trait GameWindow {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings;

    /// Returns ture if the window should close.
    fn should_close(&self) -> bool;

    /// Get the window's size
    fn get_size(&self) -> (u32, u32) {
        (self.get_settings().size[0], self.get_settings().size[1])
    }

    /// Swap buffers.
    fn swap_buffers(&self) {}

    /// When the cursor is captured,
    /// it is hidden and the cursor position does not change.
    /// Only relative mouse motion is registered.
    fn capture_cursor(&mut self, _enabled: bool) {}
    
    /// Poll a event from window's event queue.
    fn poll_event(&mut self) -> event::Event { event::NoEvent }
}

/// For use in seperate render threads, implemented by window back-end.
pub trait RenderWindow {
    /// Swap buffers.
    fn swap_buffers(&self) {}

    /// When the cursor is captured,
    /// it is hidden and the cursor position does not change.
    /// Only relative mouse motion is registered.
    fn capture_cursor(&mut self, _enabled: bool) {}
}
