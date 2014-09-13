
use graphics::ImageSize;
use graphics::vecmath::Scalar;

use event::{
    Status,
    Success,
    Running,
};

use Sprite;

/// Actions supported by Sprite
#[deriving(Clone)]
pub enum Action {
    /// duration, x, y
    ///
    /// Move sprite to specified position
    MoveTo(f64, Scalar, Scalar),
    /// duration, x, y
    ///
    /// Move sprite to specified position, relatively
    MoveBy(f64, Scalar, Scalar),
    /// duration, deg
    ///
    /// Rotate sprite to specified degree
    RotateTo(f64, Scalar),
    /// duration, deg
    ///
    /// Rotate sprite to specified degree, relatively
    RotateBy(f64, Scalar),
    /// duration, sx, sy
    ///
    /// Scale sprite to specified scale
    ScaleTo(f64, Scalar, Scalar),
    /// duration, sx, sy
    ///
    /// Scale sprite to specified scale, relatively
    ScaleBy(f64, Scalar, Scalar),
    /// Flip sprite in x direction
    FlipX(bool),
    /// Flip sprite in y direction
    FlipY(bool),
    /// Set the sprite's visibility to true
    Show,
    /// Set the sprite's visibility to false
    Hide,
    /// Toggle the sprite's visibility
    ToggleVisibility,
    /// duration, times
    Blink(f64, uint),
    /// duration
    ///
    /// Fade in the sprite, set its opacity from 0 to 1 in `dt` seconds
    FadeIn(f64),
    /// duration
    ///
    /// Fade out the sprite, set its opacity from 1 to 0 in `dt` seconds
    FadeOut(f64),
    /// duration, opacity
    ///
    /// Set the sprite's opacity to specified value in `dt` seconds
    FadeTo(f64, f64),
}

impl Action {
    /// Generate a new state from Action with specified Sprite
    pub fn to_state<I: ImageSize>(&self, sprite: &Sprite<I>) -> ActionState {
        match *self {
            MoveTo(dur, dx, dy) => {
                let (sx, sy) = sprite.position();
                MoveState(0.0, dur, sx, sy, dx, dy)
            },
            MoveBy(dur, dx, dy) => {
                let (sx, sy) = sprite.position();
                MoveState(0.0, dur, sx, sy, sx + dx, sy + dy)
            },
            RotateTo(dur, dd) => {
                let sd = sprite.rotation();
                RotateState(0.0, dur, sd, dd)
            },
            RotateBy(dur, dd) => {
                let sd = sprite.rotation();
                RotateState(0.0, dur, sd, sd + dd)
            },
            ScaleTo(dur, dsx, dsy) => {
                let (ssx, ssy) = sprite.scale();
                ScaleState(0.0, dur, ssx, ssy, dsx, dsy)
            },
            ScaleBy(dur, dsx, dsy) => {
                let (ssx, ssy) = sprite.scale();
                ScaleState(0.0, dur, ssx, ssy, ssx + dsx, ssy + dsy)
            },
            FlipX(flip_x) => {
                let flip_y = sprite.flip_y();
                FlipState(flip_x, flip_y)
            },
            FlipY(flip_y) => {
                let flip_x = sprite.flip_x();
                FlipState(flip_x, flip_y)
            },
            Show => {
                VisibilityState(true)
            },
            Hide => {
                VisibilityState(false)
            },
            ToggleVisibility => {
                let visible = sprite.visible();
                VisibilityState(!visible)
            },
            Blink(dur, times) => {
                BlinkState(0.0, dur, 0, 2 * times)
            },
            _ => {
                EmptyState
            },
        }
    }
}

/// The state of action
#[deriving(Clone)]
pub enum ActionState {
    /// past_time, duration, sx, sy, dx, dy
    MoveState(f64, f64, Scalar, Scalar, Scalar, Scalar),
    /// past_time, duration, sd, dd
    RotateState(f64, f64, Scalar, Scalar),
    /// past_time, duration, ssx, ssy, dsx, dsy
    ScaleState(f64, f64, Scalar, Scalar, Scalar, Scalar),
    /// flip_x, flip_y
    FlipState(bool, bool),
    /// visible
    VisibilityState(bool),
    /// past_time, duration, blinked_times, total_times
    BlinkState(f64, f64, uint, uint),
    /// An empty state
    EmptyState,
}

impl ActionState {
    /// Update the state and change the sprite's properties
    pub fn update<I: ImageSize>(&self, sprite: &mut Sprite<I>, dt: f64) -> (ActionState, Status, f64) {
        match *self {
            MoveState(past, dur, sx, sy, dx, dy) => {
                if past + dt >= dur {
                    sprite.set_position(dx, dy);
                    (EmptyState, Success, past + dt - dur)
                } else {
                    let factor = (past + dt) / dur;
                    sprite.set_position(sx + (dx - sx) * factor, sy + (dy - sy) * factor);
                    (MoveState(past + dt, dur, sx, sy, dx, dy),
                     Running, 0.0)
                }
            },
            RotateState(past, dur, sd, dd) => {
                if past + dt >= dur {
                    sprite.set_rotation(dd);
                    (EmptyState, Success, past + dt - dur)
                } else {
                    let factor = (past + dt) / dur;
                    sprite.set_rotation(sd + (dd - sd) * factor);
                    (RotateState(past + dt, dur, sd, dd),
                     Running, 0.0)
                }
            },
            ScaleState(past, dur, ssx, ssy, dsx, dsy) => {
                if past + dt >= dur {
                    sprite.set_scale(dsx, dsy);
                    (EmptyState, Success, past + dt - dur)
                } else {
                    let factor = (past + dt) / dur;
                    sprite.set_scale(ssx + (dsx - ssx) * factor, ssy + (dsy - ssy) * factor);
                    (ScaleState(past + dt, dur, ssx, ssy, dsx, dsy),
                     Running, 0.0)
                }
            },
            FlipState(flip_x, flip_y) => {
                sprite.set_flip_x(flip_x);
                sprite.set_flip_y(flip_y);
                (EmptyState, Success, dt)
            },
            VisibilityState(visible) => {
                sprite.set_visible(visible);
                (EmptyState, Success, dt)
            },
            BlinkState(past, dur, cur, total) => {
                let period = dur / total as f64;
                if past + dt >= (cur + 1) as f64 * period {
                    let visible = sprite.visible();
                    sprite.set_visible(!visible);
                    if past + dt >= dur {
                        (EmptyState, Success, past + dt - dur)
                    } else {
                        (BlinkState(past + dt, dur, cur + 1, total),
                         Running, 0.0)
                    }
                } else {
                    (BlinkState(past + dt, dur, cur, total),
                     Running, 0.0)
                }
            },
            // TODO:
            // match more actions
            _ => { (EmptyState, Success, dt) },
        }
    }
}

