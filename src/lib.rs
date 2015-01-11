#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![allow(unstable)]

//! Window abstraction

extern crate input;
extern crate quack;
extern crate event_loop;

use input::Input;
use quack::{ ActOn, GetFrom, SetAt, Me };

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

impl ActOn<()> for (SwapBuffers, NoWindow) {
    type Action = SwapBuffers;
    type Object = NoWindow;

    fn act_on(_: Me<Self>, _action: SwapBuffers, _window: &mut NoWindow) {}
}

impl ActOn<Option<Input>> for (PollEvent, NoWindow) {
    type Action = PollEvent;
    type Object = NoWindow;

    fn act_on(_: Me<Self>, _action: PollEvent, _window: &mut NoWindow) 
        -> Option<Input> { None }
}

impl GetFrom for (ShouldClose, NoWindow) {
    type Property = ShouldClose;
    type Object = NoWindow;

    fn get_from(_: Me<Self>, obj: &NoWindow) -> ShouldClose {
        ShouldClose(obj.should_close)
    }
}

impl GetFrom for (Size, NoWindow) {
    type Property = Size;
    type Object = NoWindow;

    fn get_from(_: Me<Self>, _obj: &NoWindow) -> Size {
        Size([0, 0])
    }
}

impl SetAt for (CaptureCursor, NoWindow) {
    type Property = CaptureCursor;
    type Object = NoWindow;

    fn set_at(_: Me<Self>, _val: CaptureCursor, _window: &mut NoWindow) {}
}

impl SetAt for (ShouldClose, NoWindow) {
    type Property = ShouldClose;
    type Object = NoWindow;

    fn set_at(
        _: Me<Self>, 
        ShouldClose(val): ShouldClose, 
        window: &mut NoWindow
    ) {
        window.should_close = val;
    }
}

impl GetFrom for (DrawSize, NoWindow) {
    type Property = DrawSize;
    type Object = NoWindow;

    fn get_from(_: Me<Self>, obj: &NoWindow) -> DrawSize {
        let Size(val) = GetFrom::get_from(Me::<(Size, NoWindow)>, obj);
        DrawSize(val)
    }
}

impl GetFrom for (Title, NoWindow) {
    type Property = Title;
    type Object = NoWindow;

    fn get_from(_: Me<Self>, obj: &NoWindow) -> Title {
        Title(obj.title.clone())
    }
}

impl SetAt for (Title, NoWindow) {
    type Property = Title;
    type Object = NoWindow;

    fn set_at(_: Me<Self>, Title(val): Title, window: &mut NoWindow) {
        window.title = val;
    }
}

impl GetFrom for (ExitOnEsc, NoWindow) {
    type Property = ExitOnEsc;
    type Object = NoWindow;

    fn get_from(_: Me<Self>, _obj: &NoWindow) -> ExitOnEsc {
        ExitOnEsc(false)
    }
}

impl SetAt for (ExitOnEsc, NoWindow) {
    type Property = ExitOnEsc;
    type Object = NoWindow;

    // Ignore attempt to exit by pressing Esc.
    fn set_at(_: Me<Self>, _: ExitOnEsc, _window: &mut NoWindow) {}
}
