use {Event, Input, Motion};

/// Stores the touch state.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum Touch {
    /// The start of touch, for example
    /// a finger pressed down on a touch screen.
    Start,
    /// The move of touch, for example
    /// a finger moving while touching a touch screen.
    Move,
    /// The end of touch, for example
    /// taking a finger away from a touch screen.
    End,
    /// The cancel of touch, for example
    /// the window loses focus.
    Cancel,
}

/// Touch arguments
///
/// The `id` might be reused for different touches that do not overlap in time.
///
/// - Coordinates are normalized to support both touch screens and trackpads
/// - Supports both 2D and 3D touch
/// - The pressure direction vector should have maximum length 1
///
/// For 2D touch the pressure is pointed the z direction.
/// Use `.pressure()` to get the pressure magnitude.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct TouchArgs {
    /// A unique identifier for touch device.
    pub device: i64,
    /// A unique identifier for touch event.
    pub id: i64,
    /// The x coordinate of the touch position, normalized 0..1.
    pub x: f64,
    /// The y coordinate of the touch position, normalized 0..1.
    pub y: f64,
    /// The z coordinate of the touch position, normalized 0..1.
    pub z: f64,
    /// The x coordinate of the touch pressure direction.
    pub px: f64,
    /// The y coordinate of the touch pressure direction.
    pub py: f64,
    /// The z coordinate of the touch pressure direction.
    pub pz: f64,
    /// Whether the touch is in 3D.
    pub is_3d: bool,
    /// The touch state.
    pub touch: Touch,
}

impl TouchArgs {
    /// Creates arguments for 2D touch.
    pub fn new(device: i64, id: i64, pos: [f64; 2], pressure: f64, touch: Touch) -> TouchArgs {
        TouchArgs {
            device: device,
            id: id,
            x: pos[0],
            y: pos[1],
            z: 0.0,
            is_3d: false,
            px: 0.0,
            py: 0.0,
            pz: pressure,
            touch: touch,
        }
    }

    /// Creates arguments for 3D touch.
    ///
    /// The pressure direction vector should have maximum length 1.
    pub fn new_3d(device: i64,
                  id: i64,
                  pos: [f64; 3],
                  pressure: [f64; 3],
                  touch: Touch)
                  -> TouchArgs {
        TouchArgs {
            device: device,
            id: id,
            x: pos[0],
            y: pos[1],
            z: pos[2],
            is_3d: true,
            px: pressure[0],
            py: pressure[1],
            pz: pressure[2],
            touch: touch,
        }
    }

    /// The position of the touch in 2D.
    pub fn position(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    /// The position of the touch in 3D.
    pub fn position_3d(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// The pressure magnitude, normalized 0..1.
    pub fn pressure(&self) -> f64 {
        (self.px * self.px + self.py * self.py + self.pz * self.pz).sqrt()
    }

    /// The pressure vector in 3D.
    pub fn pressure_3d(&self) -> [f64; 3] {
        [self.px, self.py, self.pz]
    }
}

/// When a touch is started, moved, ended or cancelled.
pub trait TouchEvent: Sized {
    /// Creates a touch event.
    fn from_touch_args(args: &TouchArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a touch event.
    fn touch<U, F>(&self, f: F) -> Option<U> where F: FnMut(&TouchArgs) -> U;
    /// Returns touch arguments.
    fn touch_args(&self) -> Option<TouchArgs> {
        self.touch(|args| args.clone())
    }
}

impl TouchEvent for Event {
    fn from_touch_args(args: &TouchArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Move(Motion::Touch(*args))))
    }

    fn touch<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&TouchArgs) -> U
    {
        match *self {
            Event::Input(Input::Move(Motion::Touch(ref args))) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_touch() {
        let pos = [0.0; 2];
        let e: Event = TouchArgs::new(0, 0, pos, 1.0, Touch::Start).into();
        let a: Option<Event> =
            TouchEvent::from_touch_args(&TouchArgs::new(0, 0, pos, 1.0, Touch::Start), &e);
        let b: Option<Event> = a.clone()
            .unwrap()
            .touch(|t| {
                TouchEvent::from_touch_args(&TouchArgs::new(t.device,
                                                            t.id,
                                                            t.position(),
                                                            t.pressure(),
                                                            Touch::Start),
                                            a.as_ref().unwrap())
            })
            .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_touch_3d() {
        use super::super::Event;

        let pos = [0.0; 3];
        let pressure = [0.0, 0.0, 1.0];
        let e: Event = TouchArgs::new_3d(0, 0, pos, pressure, Touch::Start).into();
        let a: Option<Event> =
            TouchEvent::from_touch_args(&TouchArgs::new_3d(0, 0, pos, pressure, Touch::Start), &e);
        let b: Option<Event> = a.clone()
            .unwrap()
            .touch(|t| {
                TouchEvent::from_touch_args(&TouchArgs::new_3d(t.device,
                                                               t.id,
                                                               t.position_3d(),
                                                               t.pressure_3d(),
                                                               Touch::Start),
                                            a.as_ref().unwrap())
            })
            .unwrap();
        assert_eq!(a, b);
    }
}
