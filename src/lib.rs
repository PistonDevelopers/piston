#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![feature(globs)]
#![feature(default_type_params)]

//! Window abstraction

extern crate input;
extern crate current;
extern crate event_loop;

use input::Input;
use current::{ ActOn, GetFrom, SetAt };

// Reexport everything from event_loop.
pub use event_loop::*;

/// The title of the window.
pub struct Title(pub String);

/// The anti-aliasing samples when rendering.
#[deriving(Copy)]
pub struct Samples(pub u8);

/// Whether window is opened in full screen mode.
#[deriving(Copy)]
pub struct Fullscreen(pub bool);

/// Whether to exit when pressing the Esc keyboard button.
#[deriving(Copy)]
pub struct ExitOnEsc(pub bool);

/// Whether to capture the mouse cursor.
#[deriving(Copy)]
pub struct CaptureCursor(pub bool);

/// The draw size of the window.
#[deriving(Copy)]
pub struct DrawSize(pub [u32, ..2]);

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

/// An implementation of Window that runs without a window at all.
pub struct NoWindow {
    should_close: bool,
    title: String,
}

impl NoWindow {
    /// Returns a new `NoWindow`.
    pub fn new(settings: WindowSettings) -> NoWindow {
        let title = settings.title.clone();
        NoWindow {
            should_close: false,
            title: title,
        }
    }
}

impl ActOn<NoWindow, ()> for SwapBuffers {
    fn act_on(self, _window: &mut NoWindow) {}
}

impl ActOn<NoWindow, Option<Input>> for PollEvent {
    fn act_on(self, _window: &mut NoWindow) -> Option<Input> { None }
}

impl GetFrom<NoWindow> for ShouldClose {
    fn get_from(obj: &NoWindow) -> ShouldClose {
        ShouldClose(obj.should_close)
    }
}

impl GetFrom<NoWindow> for Size {
    fn get_from(_obj: &NoWindow) -> Size {
        Size([0, 0])
    }
}

impl SetAt<NoWindow> for CaptureCursor {
    fn set_at(self, _window: &mut NoWindow) {}
}

impl SetAt<NoWindow> for ShouldClose {
    fn set_at(self, window: &mut NoWindow) {
        let ShouldClose(val) = self;
        window.should_close = val;
    }
}

impl GetFrom<NoWindow> for DrawSize {
    fn get_from(obj: &NoWindow) -> DrawSize {
        let Size(val) = GetFrom::get_from(obj);
        DrawSize(val)
    }
}

impl GetFrom<NoWindow> for Title {
    fn get_from(obj: &NoWindow) -> Title {
        Title(obj.title.clone())
    }
}

impl SetAt<NoWindow> for Title {
    fn set_at(self, window: &mut NoWindow) {
        let Title(val) = self;
        window.title = val;
    }
}

impl GetFrom<NoWindow> for ExitOnEsc {
    fn get_from(_obj: &NoWindow) -> ExitOnEsc {
        ExitOnEsc(false)
    }
}

impl SetAt<NoWindow> for ExitOnEsc {
    // Ignore attempt to exit by pressing Esc.
    fn set_at(self, _window: &mut NoWindow) {}
}
