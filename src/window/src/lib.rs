#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! Window abstraction

extern crate libc;
extern crate input;
extern crate shader_version;

use shader_version::OpenGL;
use input::Input;

/// The type of an OpenGL function address.
pub type ProcAddress = *const libc::c_void;

/// Size in pixels.
#[derive(Copy, Clone)]
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

/// Required to use the event loop.
pub trait Window {
    /// The event type emitted by `poll_event`
    type Event;

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
}

impl WindowSettings {
    /// Creates window settings with defaults.
    /// - samples: 0
    /// - fullscreen: false
    /// - exit_on_esc: false
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
        }
    }

    /// Gets title.
    pub fn get_title(&self) -> String { self.title.clone() }

    /// Sets title.
    pub fn title(mut self, value: String) -> Self {
        self.title = value;
        self
    }

    /// Gets size.
    pub fn get_size(&self) -> Size { self.size }

    /// Sets size.
    pub fn size(mut self, value: Size) -> Self {
        self.size = value;
        self
    }

    /// Gets fullscreen.
    pub fn get_fullscreen(&self) -> bool { self.fullscreen }

    /// Sets fullscreen.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.fullscreen = value;
        self
    }

    /// Gets exit on esc.
    pub fn get_exit_on_esc(&self) -> bool { self.exit_on_esc }

    /// Sets exit on esc.
    pub fn exit_on_esc(mut self, value: bool) -> Self {
        self.exit_on_esc = value;
        self
    }

    /// Gets samples.
    pub fn get_samples(&self) -> u8 { self.samples }

    /// Sets samples.
    pub fn samples(mut self, value: u8) -> Self {
        self.samples = value;
        self
    }

    /// Gets vsync.
    pub fn get_vsync(&self) -> bool { self.vsync }

    /// Sets vsync.
    pub fn vsync(mut self, value: bool) -> Self {
        self.vsync = value;
        self
    }

    /// Gets opengl.
    pub fn get_maybe_opengl(&self) -> Option<OpenGL> { self.opengl }

    /// Sets opengl.
    pub fn maybe_opengl(mut self, value: Option<OpenGL>) -> Self {
        self.opengl = value;
        self
    }

    /// Sets opengl.
    pub fn opengl(mut self, value: OpenGL) -> Self {
        self.opengl = Some(value);
        self
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

impl Window for NoWindow {
    type Event = Input;
    fn should_close(&self) -> bool { self.should_close }
    fn size(&self) -> Size { Size { width: 0, height: 0 } }
    fn swap_buffers(&mut self) {}
    fn poll_event(&mut self) -> Option<Input> { None }
    fn draw_size(&self) -> Size { self.size() }
}

impl AdvancedWindow for NoWindow {
    fn get_title(&self) -> String { self.title.clone() }
    fn set_title(&mut self, value: String) { self.title = value; }
    fn get_exit_on_esc(&self) -> bool { false }
    fn set_exit_on_esc(&mut self, _value: bool) {}
    fn set_capture_cursor(&mut self, _value: bool) {}
}
