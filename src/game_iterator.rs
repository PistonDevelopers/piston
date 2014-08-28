use time;
use std::io::timer::sleep;
use std::time::duration::Duration;

use input;
use GameWindow;

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

/// Contains the different game events.
#[deriving(Clone)]
pub enum GameEvent {
    /// Render graphics.
    Render(RenderArgs),
    /// Update physical state of the game.
    Update(UpdateArgs),
    /// Input event.
    Input(input::InputEvent),
}

#[deriving(Show)]
enum GameIteratorState {
    RenderState,
    SwapBuffersState,
    UpdateLoopState,
    HandleEventsState,
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
                        None => {
                            self.state = UpdateState;
                            // Explicitly continue because otherwise the result
                            // of this match is immediately returned.
                            continue;
                        },
                        Some(x) => Some(Input(x)),
                    }
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
