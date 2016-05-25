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
extern crate input;

use input::Input;
use std::convert::From;
use shader_version::OpenGL;

/// The type of an OpenGL function address.
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
    fn build_from_window_settings(settings: WindowSettings)
    -> Result<Self, String>;
}

// TODO: Add examples and Guide.md to the event_loop folder.
/// Trait for piston internals to interface with a window backend.
///
/// This trait defines all the behavior needed for making an event loop.
/// Event loop definitions can be found in piston::event_loop.
/// An example of a working event loop can be found in the Piston-Tutorials
/// repository, under getting-started.
/// The window trait methods are private; they're mostly for use in the abstract
/// code in the event_loop module.
pub trait Window {
    /// The event type the window uses for incoming input. Usually, this will be 
    /// event_loop::Input, but may vary between implementations if more or less
    /// information is available.
    ///
    /// For example, if a backend doesn't support mouse input because it's designed
    /// to be sent over a network, then it might use a different event type. 
    type Event;

    /// Tells the window to close or stay open. 
    /// Usually paired with a public method in the specific window implementation.
    fn set_should_close(&mut self, value: bool);

    /// Returns true if window should close.
    /// Usually paired with a public method in the specific window implementation.
    fn should_close(&self) -> bool;

    /// Gets the size of the window in user coordinates.
    /// Usually paired with a public method in the specific window implementation.
    fn size(&self) -> Size;

    /// Swaps render buffers.
    /// Client code shouldn't ever have to deal with this.
    fn swap_buffers(&mut self);

    /// Polls event from window.
    /// To read events in client code, look at the piston::event_loop::Events trait
    /// instaed.
    fn poll_event(&mut self) -> Option<Self::Event>;

    /// Gets draw size of the window.
    /// This is equal to the size of the frame buffer of the inner window,
    /// excluding the title bar and borders.
    /// This information is given to the client code through the Render event.
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
    fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets whether to exit when pressing esc.
    fn get_exit_on_esc(&self) -> bool;

    /// Sets whether to exit when pressing esc.
    fn set_exit_on_esc(&mut self, value: bool);

    /// Sets whether to exit when pressing esc. This version moves the
    /// current window data, unlike set_exit_on_esc(), so that it can be used
    /// in method chaining.
    fn exit_on_esc(mut self, value: bool) -> Self {
        self.set_exit_on_esc(value);
        self
    }

    /// Sets whether to capture/grab the cursor.
    /// This is used to lock and hide cursor to the window,
    /// for example in a first-person shooter game.
    fn set_capture_cursor(&mut self, value: bool);

    /// Sets whether to capture/grab the cursor. This version moves the
    /// current window data, unlike, unlike set_capture_cursor, so that
    /// it can be used in method chaining.
    fn capture_cursor(mut self, value: bool) -> Self {
        self.set_capture_cursor(value);
        self
    }
}

// TODO: Improve documentation of this trait.
/// Trait for OpenGL specific operations.
pub trait OpenGLWindow: Window {
    /// Returns the address of an OpenGL function if it exist, else returns null pointer.
    fn get_proc_address(&mut self, proc_name: &str) -> ProcAddress;

    /// Returns true if this context is the current context.
    fn is_current(&self) -> bool;

    /// Make this context current.
    fn make_current(&mut self);
}

/// Settings structure for window behavior.
///
/// It stores everything that needs to be customized when constructing most
/// windows. This structure makes it easy to create multiple windows with the
/// same settings, and it also makes piston's multiple backends easier to
/// implement for piston devs.
pub struct WindowSettings {
    /// Title of the window.
    title: String,
    /// The size of the window.
    size: Size,
    /// Number samples per pixel (anti-aliasing).
    samples: u8,
    /// If true, the window should be fullscreen.
    fullscreen: bool,
    /// If true, the window should exit when pressing the Esc key.
    exit_on_esc: bool,
    /// If true, enable vsync.
    vsync: bool,
    /// An optional OpenGL version.
    opengl: Option<OpenGL>,
    /// If true, sRGB is enabled.
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
	
    /// Builds window from the given settings.
    /// The return value is ambiguous, to allow for operation on multiple
    /// backends. Clients should explicitly name the return type.
    ///
    /// This function will return an error if the backend returns an error.
    /// See your backend's documentation for more details.
    pub fn build<W: BuildFromWindowSettings>(self) -> Result<W, String> {
        BuildFromWindowSettings::build_from_window_settings(self)
    }

    /// Gets the title of windows that would be made with this object.
    pub fn get_title(&self) -> String { self.title.clone() }

    /// Sets the title of windows that would be made with this object.
    /// This function moves the structure, to allow method chaining.
    pub fn title(mut self, value: String) -> Self {
        self.title = value;
        self
    }
    
    // TODO: set_title(&mut self, value: String) ?

    /// Gets the size of windows that would be made with this object.
    pub fn get_size(&self) -> Size { self.size }

    /// Sets the size windows that would be made with this object.
    /// This function moves the structure, to allow method chaining.
    pub fn size(mut self, value: Size) -> Self {
        self.size = value;
        self
    }
    
    // TODO: set_size(&mut self, value: Size) ?

    /// Gets whether windows that would be made with this object
    /// will be fullscreen.
    pub fn get_fullscreen(&self) -> bool { self.fullscreen }

    /// Sets whether windows that would be made with this object
    /// will be fullscreen. This function moves the structure, to
    /// allow method chaining.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.fullscreen = value;
        self
    }
    
    // TODO: set_fullscreen(&mut self, value: bool) ?

    /// Gets whether windows that would be made with this object
    /// should exit when the esc key is pressed.
    pub fn get_exit_on_esc(&self) -> bool { self.exit_on_esc }

    /// Sets whether windows that would be made with this object
    /// should exit when the esc key is pressed. This function moves
    /// the structure, to allow method chaining.
    pub fn exit_on_esc(mut self, value: bool) -> Self {
        self.exit_on_esc = value;
        self
    }
    
    // TODO: set_exit_on_esc(&mut self, value: bool) ?

    /// Gets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn get_samples(&self) -> u8 { self.samples }

    /// Sets the number of samples to use for anti-aliasing. This function
    /// moves the structure, to allow method chaining.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn samples(mut self, value: u8) -> Self {
        self.samples = value;
        self
    }
    
    // TODO: set_samples(&mut self, value: u8) ?

    /// Gets whether windows that would be made with this object
    /// should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn get_vsync(&self) -> bool { self.vsync }

    /// Gets whether windows that would be made with this object
    /// should use vsync. This function moves the structure, to
    /// allow method chaining.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn vsync(mut self, value: bool) -> Self {
        self.vsync = value;
        self
    }
    
    // TODO: set_vsync(&mut self, value: bool) ?

	// TODO: Document this. What does the OpenGL object mean?
    /// Gets opengl.
    pub fn get_maybe_opengl(&self) -> Option<OpenGL> { self.opengl }

	// TODO: Should we really be *setting* with an *Optional*?
    /// Sets opengl. This function moves the structure, to
    /// allow method chaining.
    pub fn maybe_opengl(mut self, value: Option<OpenGL>) -> Self {
        self.opengl = value;
        self
    }
    
    // TODO: set_maybe_opengl version?
	
	// TODO: see above TODO for maybe_opengl
    /// Sets opengl. This function moves the structure, to
    /// allow method chaining.
    pub fn opengl(mut self, value: OpenGL) -> Self {
        self.opengl = Some(value);
        self
    }

    /// Gets whether windows that would be made with this object
    /// should use sRGB for their color profiles.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    pub fn get_srgb(&self) -> bool { self.srgb }

    /// Sets whether windows that would be made with this object
    /// should use sRGB for their color profiles. This function moves
    /// the structure, to allow method chaining.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    pub fn srgb(mut self, value: bool) -> Self {
        self.srgb = value;
        self
    }
}


// TODO: This stuff doesn't belong in this file.


/// An implementation of Window that runs without a window at all.
//
/// It saves just enough information to implement the window interface,
/// but otherwise does nothing.
pub struct NoWindow {
    should_close: bool,
    title: String,
    size: Size
}

impl NoWindow {
    /// Returns a new `NoWindow`.
    pub fn new(settings: WindowSettings) -> NoWindow {
        NoWindow {
            should_close: false,
            title: settings.title,
            size: settings.size,
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
    fn build_from_window_settings(settings: WindowSettings)
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
