//! A trait that seperates the window into multiple objects.

use event;
use game_window_settings::GameWindowSettings;

/// Implemented by window back-end, required for concurrent rendering.
pub trait RenderWindow {
    /// swap buffers.
    fn swap_buffers(&self) {}
}

/// Implemented by window back-end, required for concurrent rendering.
pub trait GameLoopWindow: Send {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings;

    /// Returns ture if the window should close.
    fn should_close(&self) -> bool;

    /// Get the window's size
    fn get_size(&self) -> (u32, u32) {
        (self.get_settings().size[0], self.get_settings().size[1])
    }
    
    /// Poll a event from window's event queue.
    fn poll_event(&mut self) -> event::Event { event::NoEvent }
}

/// Seperate out into partial windows.
pub trait ConcurrentWindow<RW: RenderWindow, GLW: GameLoopWindow> {
    /// Seperate out into partial windows.
    fn get_windows(mut self) -> (RW, GLW);
}
