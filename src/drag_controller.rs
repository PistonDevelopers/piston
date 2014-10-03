//! A drag controller.

use event::{
    FocusEvent,
    GenericEvent,
    MouseCursorEvent,
    PressEvent,
    ReleaseEvent,
};
use input::{
    mouse,
    Mouse,
};

/// Describes a drag.
pub enum Drag {
    /// When the drag is interrupted by something,
    /// for example when the window is defocused.
    /// By returning true, the drag will continue when
    /// the window retrieves focus.
    InterruptDrag,
    /// Starts the drag.
    StartDrag(f64, f64),
    /// Moves the drag.
    MoveDrag(f64, f64),
    /// Ends the drag.
    EndDrag(f64, f64),
}

/// Controls dragging.
pub struct DragController {
    /// Whether to drag or not.
    pub drag: bool,
    /// The current positon of dragging.
    pub pos: [f64, ..2],
}

impl DragController {
    /// Creates a new drag controller.
    pub fn new() -> DragController {
        DragController {
            drag: false,
            pos: [0.0, 0.0],
        }
    }

    /// Handles event.
    ///
    /// Calls closure when events for dragging happen.
    /// If the drag event callback returns `false`, it will cancel dragging.
    pub fn event<E: GenericEvent>(&mut self, e: &E, f: |Drag| -> bool) {
        e.mouse_cursor(|x, y| {
            self.pos = [x, y];
            if self.drag {
                self.drag = f(MoveDrag(x, y));
            }
        });
        e.press(|button| {
            match button {
                Mouse(mouse::Left) => {
                    if !self.drag {
                        self.drag = f(StartDrag(self.pos[0], self.pos[1]));
                    }
                }
                _ => {}
            }
        });
       
        // Rest of the event are only handled when dragging. 
        if !self.drag { return; }

        e.release(|button| {
            match button {
                Mouse(mouse::Left) => {
                    if self.drag {
                        f(EndDrag(self.pos[0], self.pos[1]));
                    }
                    self.drag = false;
                }
                _ => {}
            }
        });
        e.focus(|focused| {
            if focused == false {
                self.drag = f(InterruptDrag);
            }
        });
    }
}
