#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! Window abstraction

extern crate input;
#[macro_use]
extern crate quack;
extern crate event_loop;

use input::Input;
use quack::Get;

// Reexport everything from event_loop.
pub use event_loop::*;

/// The title of the window.
pub struct Title(pub String);

/// The anti-aliasing samples when rendering.
#[derive(Copy)]
pub struct Samples(pub u8);

/// Whether window is opened in full screen mode.
#[derive(Copy)]
pub struct Fullscreen(pub bool);

/// Whether to exit when pressing the Esc keyboard button.
#[derive(Copy)]
pub struct ExitOnEsc(pub bool);

/// Whether to capture the mouse cursor.
#[derive(Copy)]
pub struct CaptureCursor(pub bool);

/// The draw size of the window.
#[derive(Copy)]
pub struct DrawSize(pub [u32; 2]);

/// Settings for window behavior.
pub struct WindowSettings {
    /// Title of the window.
    pub title: String,
    /// The size of the window.
    pub size: [u32; 2],
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

quack! {
_window: NoWindow[]
get:
    fn () -> ShouldClose [] { ShouldClose(_window.should_close) }
    fn () -> Size [] { Size([0, 0]) }
    fn () -> DrawSize [] {
        let Size(val) = _window.get();
        DrawSize(val)
    }
    fn () -> Title [] { Title(_window.title.clone()) }
    fn () -> ExitOnEsc [] { ExitOnEsc(false) }
set:
    fn (__: CaptureCursor) [] {}
    fn (val: ShouldClose) [] { _window.should_close = val.0 }
    fn (val: Title) [] { _window.title = val.0; }
    fn (__: ExitOnEsc) [] {}
action:
    fn (__: SwapBuffers) -> () [] {}
    fn (__: PollEvent) -> Option<Input> [] { None }
}

