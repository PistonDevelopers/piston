
//! A 3D camera.

use std::num::{Zero, One};
use vecmath::{
    Vector3,
    Matrix4,
    vec3_normalized_sub,
    vec3_cross,
    vec3_dot
};

/// Models a camera with position and directions.
pub struct Camera<T=f32> {
    /// The camera position.
    pub position: Vector3<T>,
    /// The up direction.
    pub up: Vector3<T>,
    /// The right direction.
    pub right: Vector3<T>,
    /// The forward direction.
    pub forward: Vector3<T>
}

/// Models camera perspective settings.
pub struct CameraPerspective<T=f32> {
    /// Field of view (in degrees).
    pub fov: T,
    /// The near clip distance.
    pub near_clip: T,
    /// The far clip distance.
    pub far_clip: T,
    /// The aspect ratio, usually set to 1.0.
    pub aspect_ratio: T,
}

impl<T: Float + FloatMath + Copy> Camera<T> {
    /// Constructs a new camera.
    ///
    /// Places the camera at [x, y, z], looking towards pozitive z.
    pub fn new(x: T, y: T, z: T) -> Camera<T> {
        let _0 = Zero::zero();
        let _1 = One::one();
        Camera {
            position: [x, y, z],
            right:   [_1, _0, _0],
            up:      [_0, _1, _0],
            forward: [_0, _0, _1]
        }
    }

    /// Computes an orthogonal matrix for the camera.
    ///
    /// This matrix can be used to transform coordinates to the screen.
    pub fn orthogonal(&self) -> Matrix4<T> {
        let p = self.position;
        let r = self.right;
        let u = self.up;
        let f = self.forward;
        let _0 = Zero::zero();
        [
            [r[0], u[0], f[0], _0],
            [r[1], u[1], f[1], _0],
            [r[2], u[2], f[2], _0],
            [-vec3_dot(r, p), -vec3_dot(u, p), -vec3_dot(f, p), One::one()]
        ]
    }

    /// Orients the camera to look at a point.
    pub fn look_at(&mut self, x: T, y: T, z: T) {
        self.forward = vec3_normalized_sub(self.position, [x, y, z]);
        self.update_right();
    }

    /// Sets yaw and pitch angle of camera in radians.
    pub fn set_yaw_pitch(&mut self, yaw: T, pitch: T) {
        let (y_s, y_c, p_s, p_c) = (yaw.sin(), yaw.cos(), pitch.sin(), pitch.cos());
        self.forward = [y_s * p_c, p_s, y_c * p_c];
        self.up = [y_s * -p_s, p_c, y_c * -p_s];
        self.update_right();
    }

    fn update_right(&mut self) {
        self.right = vec3_cross(self.up, self.forward);
    }
}

impl<T: Copy + Num + FloatMath + FromPrimitive> CameraPerspective<T> {
    /// Computes a projection matrix for the camera perspective.
    pub fn projection(&self) -> Matrix4<T> {
        let _0: T = Zero::zero();
        let _1: T = One::one();
        let _2: T = _1 + _1;
        let pi: T = Float::pi();
        let _360: T = FromPrimitive::from_int(360).unwrap();
        let f = _1 / (self.fov * (pi / _360)).tan();
        let (far, near) = (self.far_clip, self.near_clip);
        [
            [f / self.aspect_ratio, _0, _0, _0],
            [_0, f, _0, _0],
            [_0, _0, (far + near) / (near - far), -_1],
            [_0, _0, (_2 * far * near) / (near - far), _0]
        ]
    }
}


