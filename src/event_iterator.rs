use time;
use std::io::timer::sleep;
use std::time::duration::Duration;

use {
    Event,
    GenericEvent,
    Input,
    Render,
    RenderArgs,
    Update,
    UpdateArgs,
    Window,
};

use std::cmp;

#[deriving(Show)]
enum EventIteratorState {
    RenderState,
    SwapBuffersState,
    UpdateLoopState,
    HandleEventsState,
    UpdateState,
}

/// Settings for the game loop behavior.
#[deriving(Clone)]
pub struct EventSettings {
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
/// let event_settings = EventSettings {
///     updates_per_second: 120,
///     max_frames_per_second: 60,
/// };
/// let ref mut gl = Gl::new();
/// for e in EventIterator::new(&mut window, &event_settings) {
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
pub struct EventIterator<'a, W: 'a> {
    /// The game window used by iterator.
    pub window: &'a mut W,
    state: EventIteratorState,
    last_update: u64,
    last_frame: u64,
    dt_update_in_ns: u64,
    dt_frame_in_ns: u64,
    dt: f64,
}

static BILLION: u64 = 1_000_000_000;

impl<'a, W: Window<I>, I: GenericEvent> EventIterator<'a, W> {
    /// Creates a new game iterator.
    pub fn new(
        window: &'a mut W,
        settings: &EventSettings
    ) -> EventIterator<'a, W> {
        let updates_per_second: u64 = settings.updates_per_second;
        let max_frames_per_second: u64 = settings.max_frames_per_second;

        let start = time::precise_time_ns();
        EventIterator {
            window: window,
            state: RenderState,
            last_update: start,
            last_frame: start,
            dt_update_in_ns: BILLION / updates_per_second,
            dt_frame_in_ns: BILLION / max_frames_per_second,
            dt: 1.0 / updates_per_second as f64,
        }
    }
}

impl<'a, W: Window<I>, I: GenericEvent>
Iterator<Event<I>>
for EventIterator<'a, W> {
    /// Returns the next game event.
    fn next(&mut self) -> Option<Event<I>> {
        loop {
            self.state = match self.state {
                RenderState => {
                    if self.window.should_close() { return None; }

                    let start_render = time::precise_time_ns();
                    self.last_frame = start_render;

                    let (w, h) = self.window.get_size();
                    if w != 0 && h != 0 {
                        // Swap buffers next time.
                        self.state = SwapBuffersState;
                        return Some(Render(RenderArgs {
                            // Extrapolate time forward to allow smooth motion.
                            ext_dt: (start_render - self.last_update) as f64 
                                    / BILLION as f64,
                            width: w,
                            height: h,
                        }));
                    }

                    UpdateLoopState
                }
                SwapBuffersState => {
                    self.window.swap_buffers();
                    UpdateLoopState
                }
                UpdateLoopState => {
                    let current_time = time::precise_time_ns();
                    let next_frame = self.last_frame + self.dt_frame_in_ns;
                    let next_update = self.last_update + self.dt_update_in_ns;
                    let next_event = cmp::min(next_frame, next_update);
                    if next_event > current_time {
                        sleep( Duration::nanoseconds((next_event - current_time) as i64) );
                        UpdateLoopState
                    } else if next_event == next_frame {
                        RenderState
                    } else {
                        HandleEventsState
                    }
                }
                HandleEventsState => {
                    // Handle all events before updating.
                    match self.window.poll_event() {
                        None => UpdateState,
                        Some(x) => { return Some(Input(x)); },
                    }
                }
                UpdateState => {
                    self.state = UpdateLoopState;
                    self.last_update += self.dt_update_in_ns;
                    return Some(Update(UpdateArgs{ dt: self.dt }));
                }
            };
        }
    }
}
