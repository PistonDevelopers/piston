#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! Window abstraction

extern crate input;
extern crate shader_version;

use shader_version::OpenGL;
use input::Input;

/// The type of an OpenGL function address.
pub type ProcAddress = *const ();

/// Size in pixels.
#[derive(Debug, Copy, Clone)]
pub struct Size {
    /// The width in pixels.
    pub width: u32,
    /// The height in pixels.
    pub height: u32,
}

impl From<[u32; 2]> for Size {
    #[inline(always)]
    fn from(value: [u32; 2]) -> Size {
        Size { width: value[0], height: value[1] }
    }
}

impl From<(u32, u32)> for Size {
    #[inline(always)]
    fn from(value: (u32, u32)) -> Size {
        Size { width: value.0, height: value.1 }
    }
}

/// Builds window from window settings.
pub trait BuildFromWindowSettings: Sized {
    /// Builds window from window settings.
    fn build_from_window_settings(settings: &WindowSettings)
    -> Result<Self, String>;
}

/// Required to use the event loop.
pub trait Window {
    /// The event type emitted by `poll_event`
    type Event;

    /// Tells the window to close or stay open.
    fn set_should_close(&mut self, value: bool);

    /// Returns true if window should close.
    fn should_close(&self) -> bool;

    /// Gets the size of the window in user coordinates.
    fn size(&self) -> Size;

    /// Swaps render buffers.
    fn swap_buffers(&mut self);

    /// Polls event from window.
    fn poll_event(&mut self) -> Option<Self::Event>;

    /// Gets draw size of the window.
    /// This is equal to the size of the frame buffer of the inner window,
    /// excluding the title bar and borders.
    fn draw_size(&self) -> Size;
}

/// Implemented by fully supported window back-ends.
pub trait AdvancedWindow: Window + Sized {
    /// Gets a copy of the title of the window.
    fn get_title(&self) -> String;

    /// Sets the title of the window.
    fn set_title(&mut self, value: String);

    /// Sets title on window.
    fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets whether to exit when pressing esc.
    fn get_exit_on_esc(&self) -> bool;

    /// Sets whether to exit when pressing esc.
    fn set_exit_on_esc(&mut self, value: bool);

    /// Sets whether to exit when pressing esc.
    fn exit_on_esc(mut self, value: bool) -> Self {
        self.set_exit_on_esc(value);
        self
    }

    /// Sets whether to capture/grab cursor.
    /// This is used to lock and hide cursor to the window,
    /// for example in a first-person shooter game.
    fn set_capture_cursor(&mut self, value: bool);

    /// Sets whether to capture/grab cursor (see `set_capture_cursor`).
    fn capture_cursor(mut self, value: bool) -> Self {
        self.set_capture_cursor(value);
        self
    }
}

/// Trait for OpenGL specific operations.
pub trait OpenGLWindow: Window {
    /// Returns the address of an OpenGL function if it exist, else returns null pointer.
    fn get_proc_address(&mut self, proc_name: &str) -> ProcAddress;

    /// Returns true if this context is the current context.
    fn is_current(&self) -> bool;

    /// Make this context current.
    fn make_current(&mut self);
}

/// Settings for window behavior.
#[derive(Clone)]
pub struct WindowSettings {
    /// Title of the window.
    title: String,
    /// The size of the window.
    size: Size,
    /// Number samples per pixel (anti-aliasing).
    samples: u8,
    /// If true, the window is fullscreen.
    fullscreen: bool,
    /// If true, exit when pressing Esc.
    exit_on_esc: bool,
    /// If true, enable vsync.
    vsync: bool,
    /// An optional OpenGL version.
    opengl: Option<OpenGL>,
    /// Whether sRGB is enabled.
    srgb: bool,
}

impl WindowSettings {
    /// Creates window settings with defaults.
    ///
    /// - samples: 0
    /// - fullscreen: false
    /// - exit_on_esc: false
    /// - vsync: false
    /// - srgb: true
    pub fn new<T: Into<String>, S: Into<Size>>(
        title: T, size: S) -> WindowSettings
    {
        WindowSettings {
            title: title.into(),
            size: size.into(),
            samples: 0,
            fullscreen: false,
            exit_on_esc: false,
            vsync: false,
            opengl: None,
            srgb: true,
        }
    }

    /// Builds window.
    pub fn build<W: BuildFromWindowSettings>(&self) -> Result<W, String> {
        BuildFromWindowSettings::build_from_window_settings(self)
    }

    /// Gets title.
    pub fn get_title(&self) -> String { self.title.clone() }

    /// Sets title.
    pub fn set_title(&mut self, value: String) {
        self.title = value;
    }

    /// Sets title.
    pub fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets size.
    pub fn get_size(&self) -> Size { self.size }

    /// Sets size.
    pub fn set_size(&mut self, value: Size) {
        self.size = value;
    }

    /// Sets size.
    pub fn size(mut self, value: Size) -> Self {
        self.set_size(value);
        self
    }

    /// Gets fullscreen.
    pub fn get_fullscreen(&self) -> bool { self.fullscreen }

    /// Sets fullscreen.
    pub fn set_fullscreen(&mut self, value: bool) {
        self.fullscreen = value;
    }

    /// Sets fullscreen.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.set_fullscreen(value);
        self
    }

    /// Gets exit on esc.
    pub fn get_exit_on_esc(&self) -> bool { self.exit_on_esc }

    /// Sets exit on esc.
    pub fn set_exit_on_esc(&mut self, value: bool) {
        self.exit_on_esc = value;
    }

    /// Sets exit on esc.
    pub fn exit_on_esc(mut self, value: bool) -> Self {
        self.set_exit_on_esc(value);
        self
    }

    /// Gets samples.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn get_samples(&self) -> u8 { self.samples }

    /// Sets samples.
    pub fn set_samples(&mut self, value: u8) {
        self.samples = value;
    }

    /// Sets samples.
    pub fn samples(mut self, value: u8) -> Self {
        self.set_samples(value);
        self
    }

    /// Gets vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information.
    pub fn get_vsync(&self) -> bool { self.vsync }

    /// Sets vsync.
    pub fn set_vsync(&mut self, value: bool) {
        self.vsync = value;
    }

    /// Sets vsync.
    pub fn vsync(mut self, value: bool) -> Self {
        self.set_vsync(value);
        self
    }

    /// Gets OpenGL version.
    pub fn get_maybe_opengl(&self) -> Option<OpenGL> { self.opengl }

    /// Sets OpenGL version.
    pub fn set_maybe_opengl(&mut self, value: Option<OpenGL>) {
        self.opengl = value;
    }

    /// Sets OpenGL version.
    pub fn maybe_opengl(mut self, value: Option<OpenGL>) -> Self {
        self.set_maybe_opengl(value);
        self
    }

    /// Sets OpenGL version.
    pub fn set_opengl(&mut self, value: OpenGL) {
        self.opengl = Some(value);
    }

    /// Sets OpenGL version.
    pub fn opengl(mut self, value: OpenGL) -> Self {
        self.set_opengl(value);
        self
    }

    /// Gets sRGB.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    pub fn get_srgb(&self) -> bool { self.srgb }

    /// Sets sRGB.
    pub fn set_srgb(&mut self, value: bool) {
        self.srgb = value;
    }

    /// Sets sRGB.
    pub fn srgb(mut self, value: bool) -> Self {
        self.set_srgb(value);
        self
    }
}

/// An implementation of Window that runs without a window at all.
pub struct NoWindow {
    should_close: bool,
    title: String,
    size: Size
}

impl NoWindow {
    /// Returns a new `NoWindow`.
    pub fn new(settings: &WindowSettings) -> NoWindow {
        NoWindow {
            should_close: false,
            title: settings.get_title(),
            size: settings.get_size(),
        }
    }
}

impl Window for NoWindow {
    type Event = Input;
    fn should_close(&self) -> bool { self.should_close }
    fn set_should_close(&mut self, value: bool) { self.should_close = value; }
    fn size(&self) -> Size { self.size }
    fn swap_buffers(&mut self) {}
    fn poll_event(&mut self) -> Option<Input> { None }
    fn draw_size(&self) -> Size { self.size() }
}

impl BuildFromWindowSettings for NoWindow {
    fn build_from_window_settings(settings: &WindowSettings)
    -> Result<Self, String> {
        Ok(NoWindow::new(settings))
    }
}

impl AdvancedWindow for NoWindow {
    fn get_title(&self) -> String { self.title.clone() }
    fn set_title(&mut self, value: String) { self.title = value; }
    fn get_exit_on_esc(&self) -> bool { false }
    fn set_exit_on_esc(&mut self, _value: bool) {}
    fn set_capture_cursor(&mut self, _value: bool) {}
}
