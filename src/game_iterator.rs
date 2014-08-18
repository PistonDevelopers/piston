use time;
use std::io::timer::sleep;
use std::time::duration::Duration;

use GameWindow;
use keyboard;
use mouse;
use game_window;

use std::cmp;

/// Render argument.
#[deriving(Clone)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area.
    pub width: u32,
    /// The height of rendered area.
    pub height: u32,
}

/// Update argument.
#[deriving(Clone)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// Key press arguments.
#[deriving(Clone)]
pub struct KeyPressArgs {
    /// Keyboard key.
    pub key: keyboard::Key,
}

/// Key release arguments.
#[deriving(Clone)]
pub struct KeyReleaseArgs {
    /// Keyboard key.
    pub key: keyboard::Key,
}

/// Mouse press arguments.
#[deriving(Clone)]
pub struct MousePressArgs {
    /// Mouse button.
    pub button: mouse::Button,
}

/// Mouse release arguments.
#[deriving(Clone)]
pub struct MouseReleaseArgs {
    /// Mouse button.
    pub button: mouse::Button,
}

/// Mouse move arguments.
#[deriving(Clone)]
pub struct MouseMoveArgs {
    /// x in window coordinates.
    pub x: f64,
    /// y in window coordinates.
    pub y: f64,
    /// x in drawing coordinates.
    pub draw_x: f64,
    /// y in drawing coordinates.
    pub draw_y: f64,
}

/// Mouse relative move arguments.
#[deriving(Clone)]
pub struct MouseRelativeMoveArgs {
    /// Delta x in window coordinates.
    pub dx: f64,
    /// Delta y in window coordinates.
    pub dy: f64,
    /// Delta x in drawing coordinates.
    pub draw_dx: f64,
    /// Delta y in drawing coordinates.
    pub draw_dy: f64,
}

/// Mouse scroll arguments.
#[deriving(Clone)]
pub struct MouseScrollArgs {
    /// x.
    pub x: f64,
    /// y.
    pub y: f64,
}

/// Contains the different game events.
#[deriving(Clone)]
pub enum GameEvent {
    /// Render graphics.
    Render(RenderArgs),
    /// Update physical state of the game.
    Update(UpdateArgs),
    /// Pressed a keyboard key.
    KeyPress(KeyPressArgs),
    /// Released a keyboard key.
    KeyRelease(KeyReleaseArgs),
    /// Pressed a mouse button.
    MousePress(MousePressArgs),
    /// Released a mouse button.
    MouseRelease(MouseReleaseArgs),
    /// Moved mouse cursor.
    MouseMove(MouseMoveArgs),
    /// Moved mouse relative, not bounded by cursor.
    MouseRelativeMove(MouseRelativeMoveArgs),
    /// Scrolled mouse.
    MouseScroll(MouseScrollArgs)
}

#[deriving(Show)]
enum GameIteratorState {
    RenderState,
    SwapBuffersState,
    UpdateLoopState,
    HandleEventsState,
    MouseRelativeMoveState(f64, f64),
    UpdateState,
}

/// Settings for the game loop behavior.
#[deriving(Clone)]
pub struct GameIteratorSettings {
    /// The number of updates per second (UPS).
    pub updates_per_second: u64,
    /// The maximum number of frames per second (FPS target).
    pub max_frames_per_second: u64,
}

/// A game loop iterator.
///
/// *Warning: Because the iterator polls events from the window back-end,
/// it must be used on the same thread as the window back-end (usually main thread),
/// unless the window back-end supports multi-thread event polling.*
///
/// Example:
///
/// ```Rust
/// let game_iter_settings = GameIteratorSettings {
///     updates_per_second: 120,
///     max_frames_per_second: 60,
/// };
/// let ref mut gl = Gl::new();
/// for e in GameIterator::new(&mut window, &game_iter_settings) {
///     match e {
///         Render(args) => {
///             // Set the viewport in window to render graphics.
///             gl.viewport(0, 0, args.width as i32, args.height as i32);
///             // Create graphics context with absolute coordinates.
///             let c = Context::abs(args.width as f64, args.height as f64);
///             // Do rendering here.
///         },
///     }
/// }
/// ```
pub struct GameIterator<'a, W> {
    /// The game window used by iterator.
    pub game_window: &'a mut W,
    state: GameIteratorState,
    last_update: u64,
    last_frame: u64,
    dt_update_in_ns: u64,
    dt_frame_in_ns: u64,
    dt: f64,
}

static billion: u64 = 1_000_000_000;

impl<'a, W: GameWindow> GameIterator<'a, W> {
    /// Creates a new game iterator.
    pub fn new(
        game_window: &'a mut W, 
        settings: &GameIteratorSettings
    ) -> GameIterator<'a, W> {
        let updates_per_second: u64 = settings.updates_per_second;
        let max_frames_per_second: u64 = settings.max_frames_per_second;

        let start = time::precise_time_ns();
        GameIterator {
            game_window: game_window,
            state: RenderState,
            last_update: start,
            last_frame: start,
            dt_update_in_ns: billion / updates_per_second,
            dt_frame_in_ns: billion / max_frames_per_second,
            dt: 1.0 / updates_per_second as f64,
        }
    }
}

impl<'a, W: GameWindow> 
Iterator<GameEvent> 
for GameIterator<'a, W> {
    /// Returns the next game event.
    fn next(&mut self) -> Option<GameEvent> {
        loop {
            match self.state {
                RenderState => {
                    if self.game_window.should_close() { return None; }

                    let start_render = time::precise_time_ns();
                    self.last_frame = start_render;

                    let (w, h) = self.game_window.get_size();
                    if w != 0 && h != 0 {
                        // Swap buffers next time.
                        self.state = SwapBuffersState;
                        return Some(Render(RenderArgs {
                                // Extrapolate time forward to allow smooth motion.
                                ext_dt: (start_render - self.last_update) as f64 / billion as f64,
                                width: w,
                                height: h,
                            }
                        ));
                    }

                    self.state = UpdateLoopState;
                },
                SwapBuffersState => {
                    self.game_window.swap_buffers();
                    self.state = UpdateLoopState;
                },
                UpdateLoopState => {
                    let current_time = time::precise_time_ns();
                    let next_frame = self.last_frame + self.dt_frame_in_ns;
                    let next_update = self.last_update + self.dt_update_in_ns;
                    let next_event = cmp::min(next_frame, next_update);
                    if next_event > current_time {
                        sleep( Duration::nanoseconds((next_event - current_time) as i32) );
                    } else if next_event == next_frame {
                        self.state = RenderState;
                    } else {
                        self.state = HandleEventsState;
                    }
                },
                HandleEventsState => {
                    // Handle all events before updating.
                    return match self.game_window.poll_event() {
                        game_window::KeyPressed(key) => {
                            Some(KeyPress(KeyPressArgs {
                                key: key,
                            }))
                        },
                        game_window::KeyReleased(key) => {
                            Some(KeyRelease(KeyReleaseArgs {
                                key: key,
                            }))
                        },
                        game_window::MouseButtonPressed(mouse_button) => {
                            Some(MousePress(MousePressArgs {
                                button: mouse_button,
                            }))
                        },
                        game_window::MouseButtonReleased(mouse_button) => {
                            Some(MouseRelease(MouseReleaseArgs {
                                button: mouse_button,
                            }))
                        },
                        game_window::MouseMoved(x, y, relative_move) => {
                            match relative_move {
                                Some((dx, dy)) =>
                                    self.state = MouseRelativeMoveState(dx, dy),
                                None => {},
                            };
                            // Compute mouse position in drawing coordinates.
                            let (w, h) = self.game_window.get_size();
                            let (draw_w, draw_h) = self.game_window.get_draw_size();
                            let (draw_x, draw_y) = (
                                x * draw_w as f64 / w as f64, 
                                y * draw_h as f64 / h as f64
                            );
                            Some(MouseMove(MouseMoveArgs {
                                x: x,
                                y: y,
                                draw_x: x * draw_x,
                                draw_y: y * draw_y
                            }))
                        },
                        game_window::MouseScrolled(x, y) => {
                            Some(MouseScroll(MouseScrollArgs {
                                x: x,
                                y: y
                            }))
                        },
                        game_window::NoEvent => {
                            self.state = UpdateState;
                            // Explicitly continue because otherwise the result
                            // of this match is immediately returned.
                            continue;
                        },
                    }
                },
                MouseRelativeMoveState(dx, dy) => {
                    self.state = HandleEventsState;
                    // Compute mouse position in drawing coordinates.
                    let (w, h) = self.game_window.get_size();
                    let (draw_w, draw_h) = self.game_window.get_draw_size();
                    let (draw_dx, draw_dy) = (
                        dx * draw_w as f64 / w as f64, 
                        dy * draw_h as f64 / h as f64
                    );
                    return Some(MouseRelativeMove(MouseRelativeMoveArgs {
                        dx: dx,
                        dy: dy,
                        draw_dx: draw_dx,
                        draw_dy: draw_dy,
                    }));
                },
                UpdateState => {
                    self.state = UpdateLoopState;
                    self.last_update += self.dt_update_in_ns;
                    return Some(Update(UpdateArgs{
                        dt: self.dt,
                    }));
                },
            };
        }
    }
}
