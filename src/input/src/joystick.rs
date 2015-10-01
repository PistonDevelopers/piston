
//! Back-end agnostic joystick events.

use std::any::Any;

use { GenericEvent, JOYSTICK_AXIS };

/// Components of a joystick button event. Not guaranteed consistent across
/// backends.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug, Hash)]
pub struct JoystickButton {
    /// Which joystick was the button on.
    pub id: i32,
    /// Which button was pressed.
    pub button: u8,

}

impl JoystickButton {
    /// Create a new JoystickButton object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: i32, button: u8) -> Self {
        JoystickButton {
            id: id,
            button: button,
        }
    }
}

/// Components of a joystick axis move event. Not guaranteed consistent across
/// backends.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub struct JoystickAxisArgs {
    /// Which joystick moved.
    pub id: i32,
    /// The axis that moved.
    pub axis: u8,
    /// Position of the joystick. Usually [-1.0, 1.0], though backends may use
    /// a different range for various devices.
    pub position: f64
}

impl JoystickAxisArgs {
    /// Create a new JoystickAxisArgs object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: i32, axis: u8, position: f64) -> Self {
        JoystickAxisArgs {
            id: id,
            axis: axis,
            position: position,
        }
    }
}

/// The position of a joystick axis changed.
pub trait JoystickAxisEvent: Sized {
    /// Creates a joystick axis event.
    fn from_joystick_axis_args(args: JoystickAxisArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a joystick axis event.
    fn joystick_axis<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(JoystickAxisArgs) -> U;
    /// Returns joystick axis arguments.
    fn joystick_axis_args(&self) -> Option<JoystickAxisArgs> {
        self.joystick_axis(|args| args)
    }
}

impl<T: GenericEvent> JoystickAxisEvent for T {
    fn from_joystick_axis_args(args: JoystickAxisArgs, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(JOYSTICK_AXIS, &args as &Any, old_event)
    }

    fn joystick_axis<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(JoystickAxisArgs) -> U
    {
        if self.event_id() != JOYSTICK_AXIS {
            return None;
        }
        self.with_args(|any| {
            if let Some(&args) = any.downcast_ref::<JoystickAxisArgs>() {
                Some(f(args))
            } else {
                panic!("Expected JoystickAxisArgs")
            }
        })
    }
}

#[cfg(test)]
mod joystick_axis_tests {
    use super::*;

    #[test]
    fn test_input_joystick_axis() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::JoystickAxis(JoystickAxisArgs::new(0, 1, 0.9)));
        let a: Option<Input> = JoystickAxisEvent::from_joystick_axis_args(JoystickAxisArgs::new(0, 1, 0.9), &e);
        let b: Option<Input> = a.clone().unwrap().joystick_axis(|joy|
            JoystickAxisEvent::from_joystick_axis_args(JoystickAxisArgs::new(joy.id, joy.axis, joy.position), a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_joystick_axis() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::JoystickAxis(JoystickAxisArgs::new(0, 1, 0.9))));
        let a: Option<Event> = JoystickAxisEvent::from_joystick_axis_args(JoystickAxisArgs::new(0, 1, 0.9), &e);
        let b: Option<Event> = a.clone().unwrap().joystick_axis(|joy|
            JoystickAxisEvent::from_joystick_axis_args(JoystickAxisArgs::new(joy.id, joy.axis, joy.position), a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }
}
