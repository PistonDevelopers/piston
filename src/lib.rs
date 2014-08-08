#![crate_name = "ui"]
#![deny(missing_doc)]

//! A flexible standard for user interactions
//! to be used in window frameworks and widgets libraries.

/// Time difference in seconds.
pub struct DeltaTime(pub f64);

/// Describes a device.
///
/// To create a custom device, use `Device::new`.
pub struct Device(&'static str);

pub static MOUSE: Device = Device("_mouse");
pub static KEYBOARD: Device = Device("_keyboard");

impl Device {
    /// Creates a new custom device.
    ///
    /// Returns `Err(String)` if the id is reserved.
    pub fn new(id: &'static str) -> Result<Device, String> {
        let device = Device(id);
        match device {
              MOUSE
            | KEYBOARD
            => Err(format!("The device id '{}' is reserved", id)),
            _ => Ok(device)
        }
    }
}

/// Describes a button on a device.
pub struct Button {
    /// The device that contains the button.
    pub device: Device,
    /// The key that identifies the button.
    pub key: &'static str,
    /// A non-zero id if device contains more than one.
    pub id: uint,
}

/// A user input signal is a 3D data of some sort.
///
/// This can be absolute or relative depending on the usage.
pub struct Signal {
    /// The device where the signal came from.
    pub device: Device,
    /// The coordinates received from signal.
    pub xyz: [f64, ..3],
    /// A non-zero id if device contains more than one.
    pub id: uint,
}

impl Signal {
    /// Gets the `x` and `y` component.
    pub fn xy(&self) -> (f64, f64) {
        (self.xyz[0], self.xyz[1])
    }
}

/// Can be sent to a widget from window or parent widget.
pub enum Input {
    /// Press button or key.
    Press(Button),
    /// Release button or key.
    Release(Button),
    /// Repeat button or key.
    ///
    /// The frequency of repeating is implementation dependent.
    Repeat(Button),
    /// Position input event from device.
    Position(Device, [f64, ..3]),
    /// Move input event from device.
    Move(Device, [f64, ..3]),
    /// Scroll input event from device.
    Scroll(Device, [f64, ..3]),
    /// Resize input event from device.
    Resize(Device, [f64, ..3]),
    /// Orient input event from device.
    Orient(Device, [f64, ..3]),
    /// Rotate input event from device.
    Rotate(Device, [f64, ..3]),
    /// Scale input event from device.
    Scale(Device, [f64, ..3]),
    /// Select all items.
    SelectAll,
    /// Deselect all items.
    SelectNone,
    /// Select an item.
    Select(uint),
    /// Deselect an item.
    Deselect(uint),
    /// Select range of items.
    SelectRange(uint, uint),
    /// Navigate to item.
    NavigateTo(uint),
    /// Focus attention on widget.
    Focus,
    /// Shift focus away from widget.
    Defocus,
    /// Enable the widget for handling user input.
    Enable,
    /// Disable the widget from handling user input.
    Disable,
    /// Shw the child widgets.
    Expand,
    /// Hide the child widgets.
    Collapse,
    /// Play video/audio content.
    Play,
    /// Pause video/audio content.
    Pause,
    /// Skip seconds of video/audio content.
    ///
    /// This can be negative.
    Skip(DeltaTime),
    /// Set the current time of video/audio content.
    SetTime(DeltaTime),
    /// Update widget with delta time in seconds.
    Update(DeltaTime),
    /// Render the widget with computed extrapolated time for smoothness.
    Render(DeltaTime),
    /// Pastes a string in widget.
    PasteString(String),
    /// Pastes a blob of data in widget.
    PasteBlob(Vec<u8>),
    /// A unicode character, usually handled by text widgets.
    UnicodeChar(char),
}

/// Can be sent to parent of a widget.
pub enum Output {
    /// Capture a device.
    ///
    /// This is used by games to hide mouse cursor.
    Capture(Device),
    /// Free the captured device.
    Free(Device),
    /// Put a string on the clipboard.
    CopyString(String),
    /// Put a blob of data on the clipboard.
    CopyBlob(Vec<u8>),
    /// Request a size for the widget content to fit on screen.
    RequestSize([f64, ..3]),
    /// Ask to go to full screen mode.
    RequestFullscreen,
    /// Ask to go to window mode.
    RequestWindow,
    /// The number of items in widget has changed.
    ///
    /// This is used by widgets that streams data.
    Count(uint),
    /// The length of video/audio content in seconds.
    Length(DeltaTime),
    /// A warning message.
    Warning(String),
    /// A critical message.
    Alert(String),
    /// A message to notify the user.
    Notify(String),
    /// An error occured.
    Error(String),
    /// Invalid input, used by widgets that require specific format.
    Invalid(String),
}

pub mod mouse;
pub mod keyboard;
