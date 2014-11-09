//! Window abstraction

use std::cell::RefCell;
use input::InputEvent;
use current::{ Get, Modifier, Set, Usage };

use GenericEvent;

/// Whether window should close or not.
pub struct ShouldClose(pub bool);

/// Work-around trait for `Get<ShouldClose>`.
/// Used to support generic constraints.
pub trait GetShouldClose: Get<ShouldClose> {
    /// Returns whether window should close.
    fn get_should_close(&self) -> ShouldClose {
        self.get()
    }
}

impl<T: Get<ShouldClose>> GetShouldClose for T {}

/// Work-around trait for `Set<ShouldClose>`.
/// Used to support generic constraints.
/// This must be implemented for every `Modifier` impl.
pub trait SetShouldClose: Set<ShouldClose> {
    /// Sets whether window should close.
    fn set_should_close(&mut self, val: ShouldClose);
}

/// The size of the window.
pub struct Size(pub [u32, ..2]);

/// Work-around trait for `Get<Size>`.
/// Used to support generic constraints.
pub trait GetSize: Get<Size> {
    /// Returns the size of window.
    fn get_size(&self) -> Size {
        self.get()
    }
}

impl<T: Get<Size>> GetSize for T {}

/// Work-around trait for `Set<Size>`.
/// Used to support generic constraints.
/// This must be implemented for every `Modifier` impl.
pub trait SetSize: Set<Size> {
    /// Sets size of window.
    fn set_size(&mut self, val: Size);
}

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
/// This must be implemented for every `Modifier` impl.
pub trait SetTitle: Set<Title> {
    /// Sets title of window.
    fn set_title(&mut self, val: Title);
}

/// The anti-aliasing samples when rendering.
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
/// This must be implemented for every `Modifier` impl.
pub trait SetSamples: Set<Samples> {
    /// Sets antialiasing samples of window.
    fn set_samples(&mut self, val: Samples);
}

/// Whether window is opened in full screen mode.
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
/// This must be implemented for every `Modifier` impl.
pub trait SetFullscreen: Set<Fullscreen> {
    /// Sets window to fullscreen mode.
    fn set_fullscreen(&mut self, val: Fullscreen);
}

/// Whether to exit when pressing the Esc keyboard button.
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
/// This must be implemented for every `Modifier` impl.
pub trait SetExitOnEsc: Set<ExitOnEsc> {
    /// Sets exit when pressing Esc.
    fn set_exit_on_esc(&mut self, val: ExitOnEsc);
}

/// Whether to capture the mouse cursor.
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
/// This must be implemented for every `Modifier` impl.
pub trait SetCaptureCursor: Set<CaptureCursor> {
    /// Sets capture cursor.
    fn set_capture_cursor(&mut self, val: CaptureCursor);
}

/// The draw size of the window.
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
/// This must be implemented for every `Modifier` impl.
pub trait SetDrawSize: Set<DrawSize> {
    /// Sets draw size.
    fn set_draw_size(&mut self, val: DrawSize);
}

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

    impl SetShouldClose for Obj {
        fn set_should_close(&mut self, val: ShouldClose) {
            self.set_mut(val);
        }
    }

    impl Get<Size> for Obj {
        fn get(&self) -> Size { Size([0, 0]) }
    }

    impl Modifier<Obj> for Size {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetSize for Obj {
        fn set_size(&mut self, val: Size) {
            self.set_mut(val);
        }
    }

    impl Get<Title> for Obj {
        fn get(&self) -> Title { Title("hello".to_string()) }
    }

    impl Modifier<Obj> for Title {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetTitle for Obj {
        fn set_title(&mut self, val: Title) {
            self.set_mut(val);
        }
    }

    impl Get<Samples> for Obj {
        fn get(&self) -> Samples { Samples(0) }
    }

    impl Modifier<Obj> for Samples {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetSamples for Obj {
        fn set_samples(&mut self, val: Samples) {
            self.set_mut(val);
        }
    }

    impl Get<Fullscreen> for Obj {
        fn get(&self) -> Fullscreen { Fullscreen(false) }
    }

    impl Modifier<Obj> for Fullscreen {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetFullscreen for Obj {
        fn set_fullscreen(&mut self, val: Fullscreen) {
            self.set_mut(val);
        }
    }

    impl Get<ExitOnEsc> for Obj {
        fn get(&self) -> ExitOnEsc { ExitOnEsc(true) }
    }

    impl Modifier<Obj> for ExitOnEsc {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetExitOnEsc for Obj {
        fn set_exit_on_esc(&mut self, val: ExitOnEsc) {
            self.set_mut(val);
        }
    }

    impl Get<CaptureCursor> for Obj {
        fn get(&self) -> CaptureCursor { CaptureCursor(false) }
    }

    impl Modifier<Obj> for CaptureCursor {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetCaptureCursor for Obj {
        fn set_capture_cursor(&mut self, val: CaptureCursor) {
            self.set_mut(val);
        }
    }

    impl Get<DrawSize> for Obj {
        fn get(&self) -> DrawSize { DrawSize([0, 0]) }
    }

    impl Modifier<Obj> for DrawSize {
        fn modify(self, _obj: &mut Obj) {}
    }

    impl SetDrawSize for Obj {
        fn set_draw_size(&mut self, val: DrawSize) {
            self.set_mut(val);
        }
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

/// Implemented by window back-end.
pub trait Window<E: GenericEvent = InputEvent>:
    SwapBuffers
  + PollEvent<E>
  + GetShouldClose
  + GetSize
  + SetCaptureCursor {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a WindowSettings;

    /// Inform the window that it should close.
    fn close(&mut self);

    /// Get the size in drawing coordinates.
    fn get_draw_size(&self) -> (u32, u32);
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

impl Modifier<NoWindow> for CaptureCursor {
    fn modify(self, _window: &mut NoWindow) {}
}

impl SetCaptureCursor for NoWindow {
    fn set_capture_cursor(&mut self, val: CaptureCursor) {
        self.set_mut(val);
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
}

