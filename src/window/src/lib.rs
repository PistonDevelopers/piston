#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! Window storage and interfacing traits.
//!
//! The [`Window`](./trait.Window.html) trait is the minimum interface required for event loop.
//! All backends usually support this trait.
//!
//! The [`AdvancedWindow`](./trait.AdvancedWindow.html) trait is the maximum interface that can be provided,
//! while still staying consistent between backends. Not all backends implement
//! `AdvancedWindow`; check your backend's documentation to see whether it implements
//! this trait.
//!
//! The [`WindowSettings`](./struct.WindowSettings.html) structure is the preferred way of building
//! new windows in Piston. It uses the `BuildFromWindowSettings` trait,
//! which backends implement to handle window creation and setup.
//!
//! The [`OpenGLWindow`](./trait.OpenGLWindow.html) trait is used to provide low-level
//! access to OpenGL through the abstract Piston API.
//!
//! The [`Size`](./struct.Size.html) structure is used throughout Piston to store window sizes.
//! It implements some conversion traits for convenience.

extern crate shader_version;
extern crate input;

use std::convert::From;
use std::time::Duration;
use shader_version::OpenGL;
use input::Input;

pub use no_window::NoWindow;

mod no_window;

/// The type of an OpenGL function address.
///
/// Note: This is a raw pointer. It can be null!
///
/// See [`OpenGLWindow`](./trait.OpenGLWindow.html) for more information.
pub type ProcAddress = *const ();

/// Structure to store the window size.
///
/// The width and height are in *points*. On most computers, a point
/// is 1:1 with a pixel. However, this is not universally true. For example,
/// the Apple Retina Display defines 1 point to be a 2x2 square of pixels.
///
/// Normally, the consideration of points vs pixels should be left to the
/// host operating system.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    /// The width.
    pub width: u32,
    /// The height.
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

/// Structure to store the window position.
///
/// The width and height are in *points*. On most computers, a point
/// is 1:1 with a pixel. However, this is not universally true. For example,
/// the Apple Retina Display defines 1 point to be a 2x2 square of pixels.
///
/// Normally, the consideration of points vs pixels should be left to the
/// host operating system.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

impl From<[i32; 2]> for Position {
    #[inline(always)]
    fn from(value: [i32; 2]) -> Position {
        Position { x: value[0], y: value[1] }
    }
}

impl From<(i32, i32)> for Position {
    #[inline(always)]
    fn from(value: (i32, i32)) -> Position {
        Position { x: value.0, y: value.1 }
    }
}

/// Constructs a window from a [`WindowSettings`](./struct.WindowSettings.html)
/// object.
///
/// It is used by [`WindowSettings::build`](./struct.WindowSettings.html#method.build).
/// Note that the backend's implementation of this may differ from its implementation
/// of `::new()`.
pub trait BuildFromWindowSettings: Sized {
    /// Builds the window from a `WindowSettings` object.
    ///
    /// # Errors
    ///
    /// See your backend's documentation for details about what kind of
    /// error strings can be returned, and the conditions for error.
    fn build_from_window_settings(settings: &WindowSettings)
    -> Result<Self, String>;
}

/// Trait representing the minimum requirements for defining a window.
///
/// This trait defines all the behavior needed for making an event loop.
///
/// An example of a working event loop can be found in the Piston-Tutorials
/// repository under getting-started, or in the event loop examples.
pub trait Window {
    /// Tells the window to close or stay open.
    fn set_should_close(&mut self, value: bool);

    /// Returns true if the window should close.
    fn should_close(&self) -> bool;

    /// Gets the size of the window.
    fn size(&self) -> Size;

    /// Swaps render buffers.
    ///
    /// When this is set to false, this method must be called manually
    /// or through the window backend. By default it is set to true, so
    /// usually it is not needed in application code.
    fn swap_buffers(&mut self);

    /// Wait indefinitely for an input event to be available from the window.
    fn wait_event(&mut self) -> Input;

    /// Wait for an input event to be available from the window or for the
    /// specified timeout to be reached.
    ///
    /// Returns `None` only if there is no input event within the timeout.
    fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Input>;

    /// Polls an input event from the window.
    ///
    /// Return `None` if no events available.
    fn poll_event(&mut self) -> Option<Input>;

    /// Gets the draw size of the window.
    ///
    /// This is equal to the size of the frame buffer of the inner window,
    /// excluding the title bar and borders.
    ///
    /// This information is given to the client code through the
    /// [`Render`](../input/enum.Event.html) event.
    fn draw_size(&self) -> Size;
}

/// Trait representing a window with the most features that are still generic.
///
/// This trait is implemented by fully featured window back-ends. When possible,
/// reduce the trait constraint to `Window` to make the code more portable.
///
/// The `Sized` trait is required for method chaining.
pub trait AdvancedWindow: Window + Sized {
    /// Gets a copy of the title of the window.
    fn get_title(&self) -> String;

    /// Sets the title of the window.
    fn set_title(&mut self, value: String);

    /// Sets title on window.
    ///
    /// This method moves the current window data,
    /// unlike [`set_title()`](#tymethod.set_title), so
    /// that it can be used in method chaining.
    fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets whether to exit when pressing esc.
    ///
    /// Useful when prototyping.
    fn get_exit_on_esc(&self) -> bool;

    /// Sets whether to exit when pressing esc.
    ///
    /// Useful when prototyping.
    fn set_exit_on_esc(&mut self, value: bool);

    /// Sets whether to exit when pressing the Esc button.
    ///
    /// Useful when prototyping.
    ///
    /// This method moves the current window data,
    /// unlike [`set_exit_on_esc()`](#tymethod.set_exit_on_esc), so
    /// that it can be used in method chaining.
    fn exit_on_esc(mut self, value: bool) -> Self {
        self.set_exit_on_esc(value);
        self
    }

    /// Sets whether to capture/grab the cursor.
    ///
    /// This is used to lock and hide cursor to the window,
    /// for example in a first-person shooter game.
    fn set_capture_cursor(&mut self, value: bool);

    /// Sets whether to capture/grab the cursor.
    ///
    /// This method moves the current window data,
    /// unlike [`set_capture_cursor()`](#tymethod.set_capture_cursor), so
    /// that it can be used in method chaining.
    fn capture_cursor(mut self, value: bool) -> Self {
        self.set_capture_cursor(value);
        self
    }

    /// Shows the window.
    ///
    /// If the platform does not support this, it will have no effect.
    fn show(&mut self);

    /// Hides the window.
    ///
    /// If the platform does not support this, it will have no effect.
    fn hide(&mut self);

    /// Gets the position of window.
    ///
    // Returns `None` if the window no longer has a position.
    fn get_position(&self) -> Option<Position>;

    /// Sets the position of window.
    ///
    /// Has no effect if the window no longer has a position.
    fn set_position<P: Into<Position>>(&mut self, val: P);

    /// Sets the position of window.
    ///
    /// Has no effect if the window no longer has a position.
    ///
    /// This method moves the current window data,
    /// unlike [`set_position()`](#tymethod.set_position), so
    /// that it can be used in method chaining.
    fn position<P: Into<Position>>(mut self, val: P) -> Self {
        self.set_position(val);
        self
    }
}

/// Trait for OpenGL specific operations on a window.
///
/// OpenGL uses a strategy called "function pointer loading"
/// to hook up the higher level graphics APIs with the OpenGL
/// driver. Which function pointers to load depends on the
/// hardware capabilities and version of OpenGL. By using the
/// [`OpenGLWindow`](trait.OpenGLWindow.html)
/// trait, the higher level graphics API can load
/// functions from the window backend with the version set up
/// using the `WindowSettings` structure.
///
/// For more information about function pointer loading, see
/// https://www.opengl.org/wiki/Load_OpenGL_Functions
pub trait OpenGLWindow: Window {
    /// Returns the address of the specified OpenGL function if it exists.
    ///
    /// If the function does not exist, it returns a null pointer.
    fn get_proc_address(&mut self, proc_name: &str) -> ProcAddress;

    /// Returns true if this window's gl context is the current gl context.
    fn is_current(&self) -> bool;

    /// Make the window's gl context the current gl context.
    fn make_current(&mut self);
}

/// Settings structure for window behavior.
///
/// This structure stores everything that needs to be customized when
/// constructing most windows. This structure makes it easy to create multiple
/// windows with the same settings, and it also makes piston's multiple backends
/// easier to implement for piston devs.
#[derive(Clone)]
pub struct WindowSettings {
    title: String,
    size: Size,
    samples: u8,
    fullscreen: bool,
    exit_on_esc: bool,
    vsync: bool,
    opengl: Option<OpenGL>,
    srgb: bool,
    resizable: bool,
    decorated: bool,
    controllers: bool,
}

impl WindowSettings {
    /// Creates window settings with defaults.
    ///
    /// - samples: 0
    /// - fullscreen: false
    /// - exit_on_esc: false
    /// - vsync: false
    /// - srgb: true
    /// - resizable: true
    /// - decorated: true
    /// - controllers: true
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
            resizable: true,
            decorated: true,
            controllers: true,
        }
    }

    /// Builds window from the given settings.
    ///
    /// The return value is ambiguous, to allow for operation on multiple
    /// backends. Clients should explicitly name the return type. See the
    /// Guide to using Piston Windows for more info and examples.
    ///
    /// # Errors
    ///
    /// This function will return an error if your backend returns an error.
    /// See your backend's documentation on `build_from_window_settings()`
    /// for more details.
    pub fn build<W: BuildFromWindowSettings>(&self) -> Result<W, String> {
        BuildFromWindowSettings::build_from_window_settings(self)
    }

    /// Gets the title of built windows.
    pub fn get_title(&self) -> String { self.title.clone() }

    /// Sets the title of built windows.
    pub fn set_title(&mut self, value: String) {
        self.title = value;
    }

    /// Sets the title of built windows.
    ///
    /// This method moves the current window data,
    /// unlike [`set_title()`](#method.set_title),
    /// so that it can be used in method chaining.
    pub fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets the size of built windows.
    pub fn get_size(&self) -> Size { self.size }

    /// Sets the size of built windows.
    pub fn set_size(&mut self, value: Size) {
        self.size = value;
    }

    /// Sets the size of built windows.
    ///
    /// This method moves the current window data,
    /// unlike [`set_size()`](#method.set_size),
    /// so that it can be used in method chaining.
    pub fn size(mut self, value: Size) -> Self {
        self.set_size(value);
        self
    }

    /// Gets whether built windows will be fullscreen.
    pub fn get_fullscreen(&self) -> bool { self.fullscreen }

    /// Sets whether built windows will be fullscreen.
    pub fn set_fullscreen(&mut self, value: bool) {
        self.fullscreen = value;
    }

    /// Sets whether built windows will be fullscreen.
    ///
    /// This method moves the current window data,
    /// unlike [`set_fullscreen()`](#method.set_fullscreen),
    /// so that it can be used in method chaining.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.set_fullscreen(value);
        self
    }

    /// Gets whether built windows should exit when the Esc key is pressed.
    pub fn get_exit_on_esc(&self) -> bool { self.exit_on_esc }

    /// Sets whether built windows should exit when the Esc key is pressed.
    pub fn set_exit_on_esc(&mut self, value: bool) {
        self.exit_on_esc = value;
    }

    /// Sets whether built windows should exit when the Esc key is pressed.
    ///
    /// This method moves the current window data,
    /// unlike [`set_exit_on_esc()`](#method.set_exit_on_esc),
    /// so that it can be used in method chaining.
    pub fn exit_on_esc(mut self, value: bool) -> Self {
        self.set_exit_on_esc(value);
        self
    }

    /// Gets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn get_samples(&self) -> u8 { self.samples }

	/// Sets the number of samples to use for anti-aliasing.
	///
	/// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn set_samples(&mut self, value: u8) {
        self.samples = value;
    }

    /// Sets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    ///
    /// This method moves the current window data,
    /// unlike [`set_samples()`](#method.set_samples)
    /// so that it can be used in method chaining.
    pub fn samples(mut self, value: u8) -> Self {
        self.set_samples(value);
        self
    }

    /// Gets whether built windows should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn get_vsync(&self) -> bool { self.vsync }

    /// Sets whether built windows should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn set_vsync(&mut self, value: bool) {
        self.vsync = value;
    }

    /// Sets whether built windows should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    ///
    /// This method moves the current window data,
    /// unlike [`set_vsync()`](#method.set_vsync),
    /// so that it can be used in method chaining.
    pub fn vsync(mut self, value: bool) -> Self {
        self.set_vsync(value);
        self
    }

    /// Gets the OpenGL version of built windows.
    ///
    /// If None is returned, the default OpenGL version is being used. This
    /// is often a forward compatible version of OpenGL::V3_2 or
    /// higher that works with newer versions of graphics libraries.
    ///
    /// For more information about the OpenGL setting, see the
    /// [`OpenGLWindow`](trait.OpenGLWindow.html) trait.
    pub fn get_maybe_opengl(&self) -> Option<OpenGL> { self.opengl }

	/// Sets OpenGL version of built windows.
	///
	/// If None is passed, the default OpenGL version is used. This
    /// is often a forward compatible version of OpenGL::V3_2 or
    /// higher that works with newer versions of graphics libraries.
    ///
	/// For more information about the OpenGL setting, see the
    /// [`OpenGLWindow`](trait.OpenGLWindow.html) trait.
    pub fn set_maybe_opengl(&mut self, value: Option<OpenGL>) {
        self.opengl = value;
    }

    /// Sets OpenGL version of built windows.
    ///
    /// If None is passed, the default OpenGL version is used. This
    /// is often a forward compatible version of OpenGL::V3_2 or
    /// higher that works with newer versions of graphics libraries.
    ///
    /// For more information about the OpenGL setting, see the
	/// [`OpenGLWindow`](./trait.OpenGLWindow.html) trait.
	///
    /// This method moves the current window data,
    /// unlike [`set_maybe_opengl()`](#method.set_maybe_opengl),
    /// so that it can be used in method chaining.
    pub fn maybe_opengl(mut self, value: Option<OpenGL>) -> Self {
        self.set_maybe_opengl(value);
        self
    }

    /// Sets OpenGL version of built windows.
    ///
    /// For setting the OpenGL version back to default, see
    /// [`set_maybe_opengl()`](#method.set_maybe_opengl).
    ///
    /// For more information about the opengl setting, see the
	/// [`OpenGLWindow`](./trait.OpenGLWindow.html) trait.
    pub fn set_opengl(&mut self, value: OpenGL) {
        self.opengl = Some(value);
    }

    /// Sets the OpenGL version of built windows.
    ///
    /// For setting the OpenGL version back to default, see
    /// [`maybe_opengl()`](#method.maybe_opengl).
    ///
    /// For more information about the opengl setting, see the
	/// [`OpenGLWindow`](./trait.OpenGLWindow.html) trait.
    ///
    /// This method moves the current window data,
    /// unlike [`set_opengl()`](#method.set_opengl),
    /// so that it can be used in method chaining.
    pub fn opengl(mut self, value: OpenGL) -> Self {
        self.set_opengl(value);
        self
    }

    /// Gets whether built windows should use hardware accelerated color conversion.
    ///
    /// If true, the graphics hardware uses customized circuitry
    /// to convert colors from sRGB to linear color space in graphics
    /// shaders, and then converts pixel fragments back to sRGB
    /// depending on the color format of the frame buffer. This feature
    /// is supported by most graphics hardware and set to true by
    /// default.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    pub fn get_srgb(&self) -> bool { self.srgb }

    /// Sets whether built windows should use hardware accelerated color conversion.
    ///
    /// See [`get_srgb()`](#method.get_srgb) for more information about
    /// the srgb setting.
    pub fn set_srgb(&mut self, value: bool) {
        self.srgb = value;
    }

    /// Sets whether built windows should use hardware accelerated color conversion.
    ///
    /// See [`get_srgb()`](#method.get_srgb) for more information about
    /// the srgb setting.
    ///
    /// This method moves the current window data,
    /// unlike [`set_srgb()`](#method.set_srgb),
    /// so that it can be used in method chaining.
    pub fn srgb(mut self, value: bool) -> Self {
        self.set_srgb(value);
        self
    }

    /// Gets whether built windows should be resizable.
    pub fn get_resizable(&self) -> bool { self.resizable }

    /// Sets whether built windows should be resizable.
    pub fn set_resizable(&mut self, value: bool) {
        self.resizable = value;
    }

    /// Sets whether built windows should be resizable.
    ///
    /// This method moves the current window data,
    /// unlike [`set_resizable()`](#method.set_resizable),
    /// so that it can be used in method chaining.
    pub fn resizable(mut self, value: bool) -> Self {
        self.set_resizable(value);
        self
    }

    /// Gets whether built windows should be decorated.
    ///
    /// Decoration on a window refers to the Operating System's
    /// header above the window, and the window border.
    ///
    /// For more information, see
    /// https://en.wikipedia.org/wiki/Window_decoration
    pub fn get_decorated(&self) -> bool { self.decorated }

    /// Sets whether built windows should be decorated.
    ///
    /// Decoration on a window refers to the Operating System's
    /// header above the window, and the window border.
    ///
    /// For more information, see
    /// https://en.wikipedia.org/wiki/Window_decoration
    pub fn set_decorated(&mut self, value: bool) {
        self.decorated = value;
    }

    /// Sets whether built windows should be decorated.
    ///
    /// Decoration on a window refers to the Operating System's
    /// header above the window, and the window border.
    ///
    /// For more information, see
    /// https://en.wikipedia.org/wiki/Window_decoration
    ///
    /// This method moves the current window data,
    /// unlike [`set_decorated()`](#method.set_decorated),
    /// so that it can be used in method chaining.
    pub fn decorated(mut self, value: bool) -> Self {
        self.set_decorated(value);
        self
    }

    /// Gets whether built windows should listen to controller input.
    pub fn get_controllers(&self) -> bool { self.controllers }

    /// Sets whether built windows should listen to controller input.
    pub fn set_controllers(&mut self, value: bool) {
        self.controllers = value;
    }

    /// Sets whether build windows should listen to controller input.
    ///
    /// This method moves the current window data,
    /// unlike [`set_controllers()`](#method.set_controllers),
    /// so that it can be used in method chaining.
    pub fn controllers(mut self, value: bool) -> Self {
        self.set_controllers(value);
        self
    }
}
