use game_window_settings::GameWindowSettings;
use self::event::{
    Event,
    NoEvent,
};

pub mod keycode {
    pub enum KeyCode {
        UnknownKey,
        LeftKey,
        RightKey,
        UpKey,
        DownKey,
    }
}

pub mod event {
    use super::keycode::KeyCode;

    pub enum Event {
        NoEvent,
        KeyUpEvent(KeyCode),
        KeyDownEvent(KeyCode),
    }
}

/// Implemented by window back-end.
pub trait GameWindow {
    /// Creates a window.
    fn new(settings: GameWindowSettings) -> Self;
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings;

    /// Returns ture if the window should close.
    fn should_close(&self) -> bool { true }
    /// Get the window's size
    fn get_size(&self) -> (int, int) { (0, 0) }
    /// If window support double buffers, called this to tell implementation
    /// swap buffers.
    fn swap_buffers(&self) {}
    /// Poll a event from window's event queue.
    fn poll_event(&self) -> Event { NoEvent }
}
