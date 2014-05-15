//! A trait for window operations.

use game_window_settings::GameWindowSettings;
use self::event::{
    Event,
    NoEvent,
};

pub mod keycode {
//! Represent a physics key.

    /// Represent a physics key.
    #[deriving(Eq)]
    pub enum KeyCode {
        /// Key that not supported currently.
        UnknownKey,
        /// Left arrow key.
        LeftKey,
        /// Right arrow key.
        RightKey,
        /// Up arrow key.
        UpKey,
        /// Down arrow key.
        DownKey,
        /// Return
        EnterKey,
        /// Space bar
        SpaceKey,
    }
}

pub mod event {
//! MISSING DOC

    use super::keycode::KeyCode;

    /// A Event
    pub enum Event {
        /// No event occur
        NoEvent,
        /// A key was released.
        KeyReleaseEvent(KeyCode),
        /// A key was pressed.
        KeyPressEvent(KeyCode),
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
    fn poll_event(&mut self) -> Event { NoEvent }
}
