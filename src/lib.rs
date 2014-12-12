#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![feature(globs)]
#![feature(default_type_params)]

//! Window abstraction

extern crate input;
extern crate current;
extern crate event_loop;

use input::Input;
use current::{ Get, Modifier, Set };

// Reexport everything from event_loop.
pub use event_loop::*;

/// The title of the window.
pub struct Title(pub String);

/// Work-around trait for `Get<Title>`.
/// Used to support generic constraints.
pub trait GetTitle: Get<Title> {
    /// Returns the title of the window.
    fn get_title(&self) -> Title {
        self.get()
    }
}

impl<T: Get<Title>> GetTitle for T {}

/// Work-around trait for `Set<Title>`.
/// Used to support generic constraints.
pub trait SetTitle: Set<Title> {
    /// Sets title of window.
    fn set_title(&mut self, val: Title) {
        self.set_mut(val);
    }
}

impl<T: Set<Title>> SetTitle for T {}

/// The anti-aliasing samples when rendering.
#[deriving(Copy)]
pub struct Samples(pub u8);

/// Work-around trait for `Get<Samples>`.
/// Used to support generic constraints.
pub trait GetSamples: Get<Samples> {
    /// Returns the antialiasing samples when rendering.
    fn get_samples(&self) -> Samples {
        self.get()
    }
}

impl<T: Get<Samples>> GetSamples for T {}

/// Work-around trait for `Set<Samples>`.
/// Used to support generic constraints.
pub trait SetSamples: Set<Samples> {
    /// Sets antialiasing samples of window.
    fn set_samples(&mut self, val: Samples) {
        self.set_mut(val);
    }
}

impl<T: Set<Samples>> SetSamples for T {}

/// Whether window is opened in full screen mode.
#[deriving(Copy)]
pub struct Fullscreen(pub bool);

/// Work-around trait for `Get<Fullscreen>`.
/// Used to support generic constraints.
pub trait GetFullscreen: Get<Fullscreen> {
    /// Returns whether window is in full screen mode.
    fn get_fullscreen(&self) -> Fullscreen {
        self.get()
    }
}

impl<T: Get<Fullscreen>> GetFullscreen for T {}

/// Work-around trait for `Set<Fullscreen>`.
/// Used to support generic constraints.
pub trait SetFullscreen: Set<Fullscreen> {
    /// Sets window to fullscreen mode.
    fn set_fullscreen(&mut self, val: Fullscreen) {
        self.set_mut(val);
    }
}

impl<T: Set<Fullscreen>> SetFullscreen for T {}

/// Whether to exit when pressing the Esc keyboard button.
#[deriving(Copy)]
pub struct ExitOnEsc(pub bool);

/// Work-around trait for `Get<ExitOnEsc>`.
/// Used to support generic constraints.
pub trait GetExitOnEsc: Get<ExitOnEsc> {
    /// Returns whether window exits when pressing Esc.
    fn get_exit_on_esc(&self) -> ExitOnEsc {
        self.get()
    }
}

impl<T: Get<ExitOnEsc>> GetExitOnEsc for T {}

/// Work-around trait for `Set<ExitOnEsc>`.
/// Used to support generic constraints.
pub trait SetExitOnEsc: Set<ExitOnEsc> {
    /// Sets exit when pressing Esc.
    fn set_exit_on_esc(&mut self, val: ExitOnEsc) {
        self.set_mut(val);
    }
}

impl<T: Set<ExitOnEsc>> SetExitOnEsc for T {}

/// Whether to capture the mouse cursor.
#[deriving(Copy)]
pub struct CaptureCursor(pub bool);

/// Work-around trait for `Get<CaptureCursor>`.
/// Used to support generic constraints.
pub trait GetCaptureCursor: Get<CaptureCursor> {
    /// Returns whether window captures cursor.
    fn get_capture_cursor(&self) -> CaptureCursor {
        self.get()
    }
}

impl<T: Get<CaptureCursor>> GetCaptureCursor for T {}

/// Work-around trait for `Set<CaptureCursor>`.
/// Used to support generic constraints.
pub trait SetCaptureCursor: Set<CaptureCursor> {
    /// Sets capture cursor.
    fn set_capture_cursor(&mut self, val: CaptureCursor) {
        self.set_mut(val);
    }
}

impl<T: Set<CaptureCursor>> SetCaptureCursor for T {}

/// The draw size of the window.
#[deriving(Copy)]
pub struct DrawSize(pub [u32, ..2]);

/// Work-around trait for `Get<DrawSize>`.
/// Used to support generic constraints.
pub trait GetDrawSize: Get<DrawSize> {
    /// Returns the draw size of window.
    fn get_draw_size(&self) -> DrawSize {
        self.get()
    }
}

impl<T: Get<DrawSize>> GetDrawSize for T {}

/// Work-around trait for `Set<DrawSize>`.
/// Used to support generic constraints.
pub trait SetDrawSize: Set<DrawSize> {
    /// Sets draw size.
    fn set_draw_size(&mut self, val: DrawSize) {
        self.set_mut(val);
    }
}

impl<T: Set<DrawSize>> SetDrawSize for T {}

#[test]
fn test_methods() {
    use current::Modifier;

    struct Obj;

    impl Get<ShouldClose> for Obj {
        fn get(&self) -> ShouldClose { ShouldClose(false) }
    }

    impl Modifier<Obj> for ShouldClose {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<Size> for Obj {
        fn get(&self) -> Size { Size([0, 0]) }
    }

    impl Modifier<Obj> for Size {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<Title> for Obj {
        fn get(&self) -> Title { Title("hello".to_string()) }
    }

    impl Modifier<Obj> for Title {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<Samples> for Obj {
        fn get(&self) -> Samples { Samples(0) }
    }

    impl Modifier<Obj> for Samples {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<Fullscreen> for Obj {
        fn get(&self) -> Fullscreen { Fullscreen(false) }
    }

    impl Modifier<Obj> for Fullscreen {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<ExitOnEsc> for Obj {
        fn get(&self) -> ExitOnEsc { ExitOnEsc(true) }
    }

    impl Modifier<Obj> for ExitOnEsc {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<CaptureCursor> for Obj {
        fn get(&self) -> CaptureCursor { CaptureCursor(false) }
    }

    impl Modifier<Obj> for CaptureCursor {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl Get<DrawSize> for Obj {
        fn get(&self) -> DrawSize { DrawSize([0, 0]) }
    }

    impl Modifier<Obj> for DrawSize {
        fn modify(self, _obj: &mut Obj) {}
    }

    fn foo<T: GetShouldClose + SetShouldClose
            + GetSize + SetSize
            + GetTitle + SetTitle
            + GetSamples + SetSamples
            + GetFullscreen + SetFullscreen
            + GetExitOnEsc + SetExitOnEsc
            + GetCaptureCursor + SetCaptureCursor
            + GetDrawSize + SetDrawSize>(_obj: T) {}

    foo(Obj);
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
pub trait Window<E = Input>:
    SwapBuffers
  + PollEvent<E>
  + GetShouldClose + SetShouldClose
  + GetSize
  + SetCaptureCursor
  + GetDrawSize
  + GetTitle + SetTitle
  + GetExitOnEsc + SetExitOnEsc {}

impl<T:
    SwapBuffers
  + PollEvent<E>
  + GetShouldClose + SetShouldClose
  + GetSize
  + SetCaptureCursor
  + GetDrawSize
  + GetTitle + SetTitle
  + GetExitOnEsc + SetExitOnEsc,
    E> Window<E> for T {}

/// An implementation of Window that runs without a window at all.
pub struct NoWindow {
    should_close: bool,
    title: String,
}

#[test]
fn test_no_window() {
    fn foo<T: Window>() {}

    foo::<NoWindow>();
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

impl SwapBuffers for NoWindow {
    fn swap_buffers(&mut self) {}
}

impl PollEvent<Input> for NoWindow {
    fn poll_event(&mut self) -> Option<Input> { None }
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

impl Modifier<NoWindow> for CaptureCursor {
    fn modify(self, _window: &mut NoWindow) {}
}

impl Modifier<NoWindow> for ShouldClose {
    fn modify(self, window: &mut NoWindow) {
        let ShouldClose(val) = self;
        window.should_close = val;
    }
}

impl Get<DrawSize> for NoWindow {
    fn get(&self) -> DrawSize {
        let Size(val) = self.get();
        DrawSize(val)
    }
}

impl Get<Title> for NoWindow {
    fn get(&self) -> Title {
        Title(self.title.clone())
    }
}

impl Modifier<NoWindow> for Title {
    fn modify(self, window: &mut NoWindow) {
        let Title(val) = self;
        window.title = val;
    }
}

impl Get<ExitOnEsc> for NoWindow {
    fn get(&self) -> ExitOnEsc {
        ExitOnEsc(false)
    }
}

impl Modifier<NoWindow> for ExitOnEsc {
    // Ignore attempt to exit by pressing Esc.
    fn modify(self, _window: &mut NoWindow) {}
}
