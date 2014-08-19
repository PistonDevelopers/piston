#![allow(missing_doc)]

//! A First Person Shooter controller.

use std::num::{One, Zero};
use input::keyboard;
use {
    input,
    Camera,
    GameEvent,
    Input,
    Update,
};

bitflags!(flags Keys: u8 {
    static MoveForward = 0b00000001,
    static MoveBack    = 0b00000010,
    static StrafeLeft  = 0b00000100,
    static StrafeRight = 0b00001000,
    static FlyUp       = 0b00010000,
    static FlyDown     = 0b00100000
})

/// FPS controller settings.
pub struct FPSControllerSettings {
    /// Which key to press to move forward.
    pub move_forward_key: keyboard::Key,
    /// Which key to press to move backward.
    pub move_backward_key: keyboard::Key,
    /// Which key to press to strafe left.
    pub strafe_left_key: keyboard::Key,
    /// Which key to press to strafe right.
    pub strafe_right_key: keyboard::Key,
    /// Which key to press to fly up.
    pub fly_up_key: keyboard::Key,
    /// Which key to press to fly down.
    pub fly_down_key: keyboard::Key,
    /// Which key to press to move faster.
    pub move_faster_key: keyboard::Key,
}

impl FPSControllerSettings {
    /// Creates new FPS controller settings with defaults.
    pub fn default() -> FPSControllerSettings {
        FPSControllerSettings {
            move_forward_key: keyboard::W,
            move_backward_key: keyboard::S,
            strafe_left_key: keyboard::A,
            strafe_right_key: keyboard::D,
            fly_up_key: keyboard::Space,
            fly_down_key: keyboard::LShift,
            move_faster_key: keyboard::LCtrl,
        }
    }
}

/// Models a First Person Shooter (FPS) controller.
pub struct FPSController<T=f32> {
    /// The FPS controller settings.
    pub settings: FPSControllerSettings,
    /// The yaw angle (in radians).
    pub yaw: T,
    /// The pitch angle (in radians).
    pub pitch: T,
    /// The keys that are pressed.
    pub keys: Keys,
    /// The direction we are heading.
    pub direction: [T, ..3],
    /// The velocity we are moving in the direction.
    pub velocity: T,
}

impl<T: Float + FromPrimitive + Copy + FloatMath> FPSController<T> {
    /// Creates a new FPS controller.
    pub fn new(settings: FPSControllerSettings) -> FPSController<T> {
        let _0: T = Zero::zero();
        FPSController {
            settings: settings,
            yaw: _0,
            pitch: _0,
            keys: Keys::empty(),
            direction: [_0, _0, _0],
            velocity: One::one(),
        }
    }

    /// Handles game event and updates camera.
    pub fn event(&mut self, e: &GameEvent, camera: &mut Camera<T>) {
        let &FPSController {
            yaw: ref mut yaw,
            pitch: ref mut pitch,
            keys: ref mut keys,
            direction: ref mut direction,
            velocity: ref mut velocity,
            settings: ref settings,
        } = self;

        let pi: T = Float::pi();
        let sqrt2: T = Float::sqrt2();
        let _0: T = Zero::zero();
        let _1: T = One::one();
        let _2: T = FromPrimitive::from_int(2).unwrap();
        let _3: T = FromPrimitive::from_int(3).unwrap();
        let _4: T = FromPrimitive::from_int(4).unwrap();
        let _360: T = FromPrimitive::from_int(360).unwrap();
        match *e {
            Update(args) => {
                let dt: T = FromPrimitive::from_f64(args.dt).unwrap();
                let dh = dt * *velocity * _3;
                let [dx, dy, dz] = *direction;
                let (s, c) = (yaw.sin(), yaw.cos());
                camera.position = [
                    camera.position[0] + (s * dx - c * dz) * dh,
                    camera.position[1] + dy * dt * _4,
                    camera.position[2] + (s * dz + c * dx) * dh
                ];
            },
            Input(input::MouseRelativeMove(args)) => {
                let dx: T = FromPrimitive::from_f64(args.dx).unwrap();
                let dy: T = FromPrimitive::from_f64(args.dy).unwrap();
                *yaw = (*yaw - dx / _360 * pi / _4) % (_2 * pi);
                *pitch = *pitch + dy / _360 * pi / _4;
                *pitch = (*pitch).min(pi / _2).max(-pi / _2);
                camera.set_yaw_pitch(*yaw, *pitch);
            },
            Input(input::KeyPress(args)) => {
                let [dx, dy, dz] = *direction;
                let sgn = |x: T| if x == _0 { _0 } else { x.signum() };
                let set = |k, x: T, y: T, z: T| {
                    let (x, z) = (sgn(x), sgn(z));
                    let (x, z) = if x != _0 && z != _0 {
                        (x / sqrt2, z / sqrt2)
                    } else {
                        (x, z)
                    };
                    *direction = [x, y, z];
                    keys.insert(k);
                };
                match args.key {
                    x if x == settings.move_forward_key => set(MoveForward, -_1, dy, dz),
                    x if x == settings.move_backward_key => set(MoveBack, _1, dy, dz),
                    x if x == settings.strafe_left_key => set(StrafeLeft, dx, dy, _1),
                    x if x == settings.strafe_right_key => set(StrafeRight, dx, dy, -_1),
                    x if x == settings.fly_up_key => set(FlyUp, dx, _1, dz),
                    x if x == settings.fly_down_key => set(FlyDown, dx, -_1, dz),
                    x if x == settings.move_faster_key => *velocity = _2,
                    _ => {}
                }
            },
            Input(input::KeyRelease(args)) => {
                let [dx, dy, dz] = *direction;
                let sgn = |x: T| if x == _0 { _0 } else { x.signum() };
                let set = |x: T, y: T, z: T| {
                    let (x, z) = (sgn(x), sgn(z));
                    let (x, z) = if x != _0 && z != _0 {
                        (x / sqrt2, z / sqrt2)
                    } else {
                        (x, z)
                    };
                    *direction = [x, y, z];
                };
                let release = |key, rev_key, rev_val| {
                    keys.remove(key);
                    if keys.contains(rev_key) { rev_val } else { _0 }
                };
                match args.key {
                    x if x == settings.move_forward_key => set(release(MoveForward, MoveBack, _1), dy, dz),
                    x if x == settings.move_backward_key => set(release(MoveBack, MoveForward, -_1), dy, dz),
                    x if x == settings.strafe_left_key => set(dx, dy, release(StrafeLeft, StrafeRight, -_1)),
                    x if x == settings.strafe_right_key => set(dx, dy, release(StrafeRight, StrafeLeft, _1)),
                    x if x == settings.fly_up_key => set(dx, release(FlyUp, FlyDown, -_1), dz),
                    x if x == settings.fly_down_key => set(dx, release(FlyDown, FlyUp, _1), dz),
                    x if x == settings.move_faster_key => *velocity = _1,
                    _ => {}
                }
            },
            _ => {},
        }
    }
}

