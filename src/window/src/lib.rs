#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! Window storage and interfacing traits.
//!
//! The Window, AdvancedWindow, BuildFromWindowSettings traits are used by
//! Piston to interface with its many backends. They aren't generally useful
//! for the average piston user.
//!
//! The WindowSettings structure is the preferred, ideomatic way of building
//! new windows in Piston. 
//!
//! The OpenGLWindow trait is used for windows that are based on OpenGL, to
//! provide the Piston API with some extra information.
//!
//! Finally, the Size structure is used throughout Piston to encode windowsizes.
//! A few convenience conversions are included. 

extern crate shader_version;

use std::convert::From;
use shader_version::OpenGL;

/// The type of an OpenGL function address.
///
/// Note: This is a raw pointer. It can be null!
pub type ProcAddress = *const ();

/// Structure to store the window size.
#[derive(Debug, Copy, Clone)]
pub struct Size {
    /// The width in pixels.
    pub width: u32,
    /// The height in pixels.
    pub height: u32,
}

/// Convinience conversion between an array of size two, and a size structure.
impl From<[u32; 2]> for Size {
    #[inline(always)]
    fn from(value: [u32; 2]) -> Size {
        Size { width: value[0], height: value[1] }
    }
}

/// Convinience conversion between a tuple of (width,height), and a size structure.
impl From<(u32, u32)> for Size {
    #[inline(always)]
    fn from(value: (u32, u32)) -> Size {
        Size { width: value.0, height: value.1 }
    }
}

/// This trait indicates that a window can be built from a WindowSettings object.
pub trait BuildFromWindowSettings: Sized {
    /// Builds the window from a window settings object.
    ///
    /// # Errors
    ///
    /// See your backend's documentation for details about what kind of
    /// error strings can be returned, and the conditions for error.
    fn build_from_window_settings(settings: WindowSettings)
    -> Result<Self, String>;
}

/// Trait for piston internals to interface with a window backend.
///
/// This trait defines all the behavior needed for making an event loop.
/// Event loop definitions can be found in [`piston::event_loop`]
/// (../event_loop/index.html).
/// An example of a working event loop can be found in the Piston-Tutorials
/// repository, under getting-started, or in the event loop examples.
/// The window trait methods are private; they're mostly for use in the abstract
/// code in the `event_loop` module.
pub trait Window {
    /// The event type the window uses for incoming input.
    /// 
    /// Usually, this will be event_loop::Input, but may vary
    /// between implementations if more or less information is available.
    ///
    /// For example, if a backend doesn't support mouse input because it's designed
    /// to be sent over a network, then it might use a different event type. 
    type Event;

    /// Tells the window to close or stay open. 
    ///
    /// Usually paired with a public method in the specific window implementation.
    fn set_should_close(&mut self, value: bool);

    /// Returns true if window should close.
    ///
    /// Usually paired with a public method in the specific window implementation.
    fn should_close(&self) -> bool;

    /// Gets the size of the window in user coordinates.
    ///
    /// Usually paired with a public method in the specific window implementation.
    fn size(&self) -> Size;

    /// Swaps render buffers.
    ///
    /// Client code shouldn't ever have to deal with this.
    fn swap_buffers(&mut self);

    /// Polls event from window.
    ///
    /// To read events in client code, look at the
    /// [`Events`](../event_loop/trait.Events.html) trait instaed.
    fn poll_event(&mut self) -> Option<Self::Event>;

    /// Gets draw size of the window.
    ///
    /// This is equal to the size of the frame buffer of the inner window,
    /// excluding the title bar and borders.
    /// This information is given to the client code through the 
    /// [`Render`](../input/enum.Event.html) event.
    fn draw_size(&self) -> Size;
}

/// Trait for piston internals to interface with a window backend with extra capabilities.
///
/// This trait is implemented by fully supported window back-ends. It provides some
/// extra information for the event_loop and input modules to work with. Normally,
/// all of these methods have 
pub trait AdvancedWindow: Window + Sized {
    /// Gets a copy of the title of the window.
    fn get_title(&self) -> String;

    /// Sets the title of the window.
    fn set_title(&mut self, value: String);

    /// Sets title on window.
    ///
    /// This version moves the current window data, 
    /// unlike [`set_title()`](#method.set_title), so 
    /// that it can be used in method chaining.
    fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets whether to exit when pressing esc.
    fn get_exit_on_esc(&self) -> bool;

    /// Sets whether to exit when pressing esc.
    fn set_exit_on_esc(&mut self, value: bool);

    /// Sets whether to exit when pressing the Esc button.
    /// 
    /// This version moves the current window data, 
    /// unlike [`set_exit_on_esc()`](#method.set_exit_on_esc), so 
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
    /// This version moves the current window data, 
    /// unlike [`set_capture_cursor()`](#method.set_capture_cursor), so
    /// that it can be used in method chaining.
    fn capture_cursor(mut self, value: bool) -> Self {
        self.set_capture_cursor(value);
        self
    }
}

/// Trait for OpenGL specific operations.
///
/// For backends that support OpenGL, this trait allows client code to
/// access the raw OpenGL context.
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
/// It stores everything that needs to be customized when constructing most
/// windows. This structure makes it easy to create multiple windows with the
/// same settings, and it also makes piston's multiple backends easier to
/// implement for piston devs.
#[derive(Clone)]
pub struct WindowSettings {
    title: String,
    size: Size,
    /// Number samples per pixel.
    ///
    /// Used during anti-aliasing.
    samples: u8,
    /// If true, the window should be fullscreen.
    fullscreen: bool,
    /// If true, the window should exit when pressing the Esc key.
    exit_on_esc: bool,
    vsync: bool,
    /// An optional OpenGL instance.
    opengl: Option<OpenGL>,
    /// If true, sRGB is enabled.
    srgb: bool,
    resizable: bool,
    decorated: bool,
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
    pub fn build<W: BuildFromWindowSettings>(self) -> Result<W, String> {
        BuildFromWindowSettings::build_from_window_settings(self)
    }

    /// Gets the title of windows that would be made with this object.
    pub fn get_title(&self) -> String { self.title.clone() }

    /// Sets title.
    pub fn set_title(&mut self, value: String) {
        self.title = value;
    }

    /// Sets the title of windows that would be made with this object.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }
    
    /// Gets the size of windows that would be made with this object.
    pub fn get_size(&self) -> Size { self.size }

    /// Sets size.
    pub fn set_size(&mut self, value: Size) {
        self.size = value;
    }

    /// Sets the size windows that would be made with this object.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn size(mut self, value: Size) -> Self {
        self.set_size(value);
        self
    }
    
    /// Gets whether windows that would be made with this object
    /// will be fullscreen.
    pub fn get_fullscreen(&self) -> bool { self.fullscreen }

    /// Sets fullscreen.
    pub fn set_fullscreen(&mut self, value: bool) {
        self.fullscreen = value;
    }

    /// Sets whether windows that would be made with this object
    /// will be fullscreen.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.set_fullscreen(value);
        self
    }
    
    /// Gets whether windows that would be made with this object
    /// should exit when the Esc key is pressed.
    pub fn get_exit_on_esc(&self) -> bool { self.exit_on_esc }

    /// Sets exit on esc.
    pub fn set_exit_on_esc(&mut self, value: bool) {
        self.exit_on_esc = value;
    }

    /// Sets whether windows that would be made with this object
    /// should exit when the Esc key is pressed. 
    ///
    /// This version moves the current window data,
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
    pub fn set_samples(&mut self, value: u8) {
        self.samples = value;
    }

    /// Sets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn samples(mut self, value: u8) -> Self {
        self.set_samples(value);
        self
    }

    /// Gets whether windows that would be made with this object
    /// should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn get_vsync(&self) -> bool { self.vsync }

    /// Sets vsync.
    pub fn set_vsync(&mut self, value: bool) {
        self.vsync = value;
    }

    /// Gets whether windows that would be made with this object
    /// should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn vsync(mut self, value: bool) -> Self {
        self.set_vsync(value);
        self
    }
    
	// TODO: Document this. What does the OpenGL object mean?
    /// Gets the opengl instance.
    pub fn get_maybe_opengl(&self) -> Option<OpenGL> { self.opengl }
	
	/// Sets OpenGL version.
    pub fn set_maybe_opengl(&mut self, value: Option<OpenGL>) {
        self.opengl = value;
    }
	
    /// Sets opengl instance, or removes it if passed None.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn maybe_opengl(mut self, value: Option<OpenGL>) -> Self {
        self.set_maybe_opengl(value);
        self
    }
	
    /// Sets OpenGL version.
    pub fn set_opengl(&mut self, value: OpenGL) {
        self.opengl = Some(value);
    }
    
    /// Sets the opengl instance.
    ///
    /// For removing the opengl instance, see
    /// [`maybe_opengl()`](#method.maybe_opengl).
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn opengl(mut self, value: OpenGL) -> Self {
        self.set_opengl(value);
        self
    }
    
    // TODO: set_opengl version?

    /// Gets whether windows that would be made with this object
    /// should use sRGB for their color profiles.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    pub fn get_srgb(&self) -> bool { self.srgb }

    /// Sets sRGB.
    pub fn set_srgb(&mut self, value: bool) {
        self.srgb = value;
    }

    /// Sets whether windows that would be made with this object
    /// should use sRGB for their color profiles.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    ///
    /// This version moves the current window data,
    /// so that it can be used in method chaining.
    pub fn srgb(mut self, value: bool) -> Self {
        self.set_srgb(value);
        self
    }

    /// Gets whether window should be resizable.
    pub fn get_resizable(&self) -> bool { self.resizable }

    /// Sets whether window should be resizable.
    pub fn set_resizable(&mut self, value: bool) {
        self.resizable = value;
    }

    /// Sets whether window should be resizable.
    pub fn resizable(mut self, value: bool) -> Self {
        self.set_resizable(value);
        self
    }

    /// Gets whether window should be decorated.
    pub fn get_decorated(&self) -> bool { self.decorated }

    /// Sets whether window should be decorated.
    pub fn set_decorated(&mut self, value: bool) {
        self.decorated = value;
    }

    /// Sets whether window should be decorated.
    pub fn decorated(mut self, value: bool) -> Self {
        self.set_decorated(value);
        self
    }
    
    // TODO: set_srgb version?
}

