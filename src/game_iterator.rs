use time;
use std::io::timer::sleep;

use GameWindow;
use keyboard;
use mouse;
use event;

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
    /// x.
    pub x: f64,
    /// y.
    pub y: f64,
}

/// Mouse relative move arguments.
#[deriving(Clone)]
pub struct MouseRelativeMoveArgs {
    /// Delta x.
    pub dx: f64,
    /// Delta y.
    pub dy: f64,
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

enum GameIteratorState {
    RenderState,
    SwapBuffersState,
    PrepareUpdateLoopState,
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
/// Example:
///
/// ```Rust
/// let game_iter_settings = GameIteratorSettings {
///     updates_per_second: 120,
///     max_frames_per_second: 60,
/// };
/// let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
/// loop {
///     match game_iter.next() {
///         None => break,
///         Some(mut e) => match e {
///             Render(ref mut args) => {
///                 // Create graphics context with absolute coordinates.
///                 let c = Context::abs(args.width as f64, args.height as f64);
///                 // Do rendering here.
///             },
///             _ => {},       
///         },
///     }
/// }
/// ```
pub struct GameIterator<'a, W> {
    /// The game window used by iterator.
    pub game_window: &'a mut W,
    state: GameIteratorState,
    last_update: u64,
    update_time_in_ns: u64,
    dt: f64,
    min_updates_per_frame: u64,
    min_ns_per_frame: u64,
    start_render: u64,
    next_render: u64,
    updated: u64,
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
            update_time_in_ns: billion / updates_per_second,
            dt: 1.0 / updates_per_second as f64,
            // You can make this lower if needed.
            min_updates_per_frame: updates_per_second / max_frames_per_second,
            min_ns_per_frame: billion / max_frames_per_second,
            start_render: start,
            next_render: start,
            updated: 0,
        }
    }
}

impl<'a, W: GameWindow> 
Iterator<GameEvent> 
for GameIterator<'a, W> {
    /// Returns the next game event.
    fn next(&mut self) -> Option<GameEvent> {
        match self.state {
            RenderState => {
                if self.game_window.should_close() { return None; }

                self.start_render = time::precise_time_ns();
                // Rendering code
                let (w, h) = self.game_window.get_size();
                if w != 0 && h != 0 {
                    // Swap buffers next time.
                    self.state = SwapBuffersState;
                    return Some(Render(RenderArgs {
                            // Extrapolate time forward to allow smooth motion.
                            // 'start_render' is always bigger than 'last_update'.
                            ext_dt: (self.start_render - self.last_update) as f64 / billion as f64,
                            width: w,
                            height: h,
                        }
                    ));
                }


                self.state = PrepareUpdateLoopState;
                return self.next();
            },
            SwapBuffersState => {
                self.game_window.swap_buffers();
                self.state = PrepareUpdateLoopState;
                return self.next();
            },
            PrepareUpdateLoopState => {
                self.updated = 0;
                self.next_render = self.start_render + self.min_ns_per_frame;
                self.state = UpdateLoopState;
                return self.next();
            },
            UpdateLoopState => {
                let got_min_updates = self.updated < self.min_updates_per_frame;
                let got_time_to_update = time::precise_time_ns() < self.next_render;
                let before_next_frame = self.last_update + self.update_time_in_ns < self.next_render;

                if ( got_time_to_update || got_min_updates ) && before_next_frame {
                    self.state = HandleEventsState;
                    return self.next();
                }

                // Wait if possible.
                // Convert to ms because that is what the sleep function takes.
                let t = (self.next_render - time::precise_time_ns() ) / 1_000_000;
                if t > 1 && t < 1000000 { // The second half just checks if it overflowed,
                                          // which tells us that t should have been negative
                                          // and we are running slow and shouldn't sleep.
                    sleep( t );
                }
                self.state = RenderState;
                return self.next();
            },
            HandleEventsState => {
                // Handle all events before updating.
                return match self.game_window.poll_event() {
                    event::KeyPressed(key) => {
                        Some(KeyPress(KeyPressArgs {
                            key: key,
                        }))
                    },
                    event::KeyReleased(key) => {
                        Some(KeyRelease(KeyReleaseArgs {
                            key: key,
                        }))
                    },
                    event::MouseButtonPressed(mouse_button) => {
                        Some(MousePress(MousePressArgs {
                            button: mouse_button,
                        }))
                    },
                    event::MouseButtonReleased(mouse_button) => {
                        Some(MouseRelease(MouseReleaseArgs {
                            button: mouse_button,
                        }))
                    },
                    event::MouseMoved(x, y, relative_move) => {
                        match relative_move {
                            Some((dx, dy)) =>
                                self.state = MouseRelativeMoveState(dx, dy),
                            None => {},
                        };
                        Some(MouseMove(MouseMoveArgs {
                            x: x,
                            y: y,
                        }))
                    },
                    event::MouseScrolled(x, y) => {
                        Some(MouseScroll(MouseScrollArgs { 
                            x: x, 
                            y: y
                        }))
                    },
                    event::NoEvent => {
                        self.state = UpdateState;
                        self.next()
                    },
                }
            },
            MouseRelativeMoveState(dx, dy) => {
                self.state = HandleEventsState;
                return Some(MouseRelativeMove(MouseRelativeMoveArgs {
                    dx: dx,
                    dy: dy,
                }));
            },
            UpdateState => {
                self.updated += 1;
                self.state = UpdateLoopState;
                self.last_update += self.update_time_in_ns;
                return Some(Update(UpdateArgs{
                    dt: self.dt,
                }));
            },
        };

        /*
        // copied.

        while !self.should_close(game_window) {

            let start_render = time::precise_time_ns();

            // Rendering code
            let (w, h) = game_window.get_size();
            if w != 0 && h != 0 {
                self.viewport(game_window);
                let mut gl = Gl::new(&mut gl_data, asset_store);
                bg.clear(&mut gl);
                // Extrapolate time forward to allow smooth motion.
                // 'now' is always bigger than 'last_update'.
                let ext_dt = (start_render - last_update) as f64 / billion as f64;
                self.render(
                    ext_dt,
                    &context
                        .trans(-1.0, 1.0)
                        .scale(2.0 / w as f64, -2.0 / h as f64)
                        .store_view(),
                    &mut gl
                        );
                self.swap_buffers(game_window);
            }

            let next_render = start_render + min_ns_per_frame;

            // Update gamestate
            let mut updated = 0;

            while // If we haven't reached the required number of updates yet
                  ( updated < min_updates_per_frame ||
                    // Or we have the time to update further
                    time::precise_time_ns() < next_render ) &&
                  //And we haven't already progressed time to far
                  last_update + update_time_in_ns < next_render {

                self.handle_events(game_window, asset_store);
                self.update(dt, asset_store);

                updated += 1;
                last_update += update_time_in_ns;
            }

            // Wait if possible

            let t = (next_render - time::precise_time_ns() ) / 1_000_000;
            if t > 1 && t < 1000000 { // The second half just checks if it overflowed,
                                      // which tells us that t should have been negative
                                      // and we are running slow and shouldn't sleep.
                sleep( t );
            }

        }
        */
    }
}
