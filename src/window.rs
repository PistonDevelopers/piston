//! Window abstraction

use std::cell::RefCell;
use input::InputEvent;
use current::{ Get, Usage };

use GenericEvent;

/// Whether window should close or not.
pub struct ShouldClose(pub bool);

/// The size of the window.
pub struct Size(pub [u32, ..2]);

/// Implemented by windows that can swap buffers.
pub trait SwapBuffers {
    /// Swaps the buffers.
    fn swap_buffers(&mut self);
}

impl<'a, W: 'a + SwapBuffers> SwapBuffers for Usage<'a, W> {
    #[inline(always)]
    fn swap_buffers(&mut self) {
        self.with_unwrap(|window: &RefCell<W>| {
            window.borrow_mut().deref_mut().swap_buffers()
        })
    }
}

impl<'a, W: 'a + SwapBuffers> SwapBuffers for &'a RefCell<W> {
    #[inline(always)]
    fn swap_buffers(&mut self) {
        self.borrow_mut().deref_mut().swap_buffers()
    }
}

/// Implemented by windows that can pull events.
pub trait PollEvent<E: GenericEvent> {
    /// Polls event from window.
    fn poll_event(&mut self) -> Option<E>;
}

impl<'a, W: 'a + PollEvent<I>, I: GenericEvent> PollEvent<I> for Usage<'a, W> {
    #[inline(always)]
    fn poll_event(&mut self) -> Option<I> {
        self.with_unwrap(|window: &RefCell<W>| {
            window.borrow_mut().deref_mut().poll_event()
        })
    }
}

impl<'a, W: 'a + PollEvent<I>, I: GenericEvent> PollEvent<I> for &'a RefCell<W> {
    #[inline(always)]
    fn poll_event(&mut self) -> Option<I> {
        self.borrow_mut().deref_mut().poll_event()
    }
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

/// Work-around trait for `Get<ShouldClose>`.
/// Used to support generic constraints.
pub trait GetShouldClose: Get<ShouldClose> {
    /// Returns whether window should close.
    fn get_should_close(&self) -> ShouldClose {
        self.get()
    }
}

impl<T: Get<ShouldClose>> GetShouldClose for T {}

/// Work-around trait for `Get<Size>`.
/// Used to support generic constraints.
pub trait GetSize: Get<Size> {
    /// Returns the size of window.
    fn get_size(&self) -> Size {
        self.get()
    }
}

impl<T: Get<Size>> GetSize for T {}

/// Implemented by window back-end.
pub trait Window<E: GenericEvent = InputEvent>:
    SwapBuffers
  + PollEvent<E>
  + GetShouldClose
  + GetSize {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a WindowSettings;

    /// Inform the window that it should close.
    fn close(&mut self);

    /// Get the size in drawing coordinates.
    fn get_draw_size(&self) -> (u32, u32);

    /// When the cursor is captured,
    /// it is hidden and the cursor position does not change.
    /// Only relative mouse motion is registered.
    fn capture_cursor(&mut self, _enabled: bool);
}

/// An implementation of Window that runs without a window at all.
pub struct NoWindow {
    settings: WindowSettings,
    should_close: bool
}

impl NoWindow {
    /// Returns a new `NoWindow`.
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

impl Get<ShouldClose> for NoWindow {
    fn get(&self) -> ShouldClose {
        ShouldClose(self.should_close)
    }
}

impl Get<Size> for NoWindow {
    fn get(&self) -> Size {
        Size([0, 0])
    }
}

impl Window<InputEvent> for NoWindow {
     fn get_settings<'a>(&'a self) -> &'a WindowSettings {
        &self.settings
     }

    fn close(&mut self) {
        self.should_close = true
    }

    fn get_draw_size(&self) -> (u32, u32) {
        let Size([w, h]) = self.get_size();
        (w, h)
    }

    fn capture_cursor(&mut self, _enabled: bool) {}
}
