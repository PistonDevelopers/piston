//! Event identifiers.

/// Event id for after render event.
pub const AFTER_RENDER: EventId = EventId("piston/after_render");
/// Event id for controller axis event.
pub const CONTROLLER_AXIS: EventId = EventId("piston/controller_axis");
/// Event id for cursor event.
pub const CURSOR: EventId = EventId("piston/cursor");
/// Event id for focus event.
pub const FOCUS: EventId = EventId("piston/focus");
/// Event id for close event.
pub const CLOSE: EventId = EventId("piston/close");
/// Event id for idle event.
pub const IDLE: EventId = EventId("piston/idle");
/// Event id for mouse scroll event.
pub const MOUSE_SCROLL: EventId = EventId("piston/mouse_scroll");
/// Event id for mouse relative event.
pub const MOUSE_RELATIVE: EventId = EventId("piston/mouse_relative");
/// Event id for mouse cursor event.
pub const MOUSE_CURSOR: EventId = EventId("piston/mouse_cursor");
/// Event id for button event.
pub const BUTTON: EventId = EventId("piston/button");
/// Event id for render event.
pub const RENDER: EventId = EventId("piston/render");
/// Event id for resize event.
pub const RESIZE: EventId = EventId("piston/resize");
/// Event id for text event.
pub const TEXT: EventId = EventId("piston/text");
/// Event id for touch event.
pub const TOUCH: EventId = EventId("piston/touch");
/// Event id for update event.
pub const UPDATE: EventId = EventId("piston/update");
/// Event id for file drag event.
pub const FILE_DRAG: EventId = EventId("piston/file_drag");

/// Used to identify events arguments provided by traits.
///
/// Use format `<api>/<event>` to avoid naming collision.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct EventId(pub &'static str);
