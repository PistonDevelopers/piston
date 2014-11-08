use time;
use std::io::timer::sleep;
use std::time::duration::Duration;
use current::{ Modifier };
use window::{
    PollEvent, 
    ShouldClose, GetShouldClose,
    Size, GetSize,
    SwapBuffers,
};

use {
    Event,
    GenericEvent,
    Input,
    Render,
    RenderArgs,
    Update,
    UpdateArgs,
};

use std::cmp;

#[deriving(Show)]
enum EventsState {
    RenderState,
    SwapBuffersState,
    UpdateLoopState,
    HandleEventsState,
    UpdateState,
}

/// The number of updates per second.
/// If the event loop lags, it will try to catch up.
pub struct Ups(pub u64);

impl<W> Modifier<Events<W>> for Ups {
    fn modify(self, events: &mut Events<W>) {
        let Ups(frames) = self;
        events.dt_update_in_ns = BILLION / frames;
    }
}

/// The maximum number of frames per second.
/// Next frame is always scheduled from the previous frame.
pub struct MaxFps(pub u64);

impl<W> Modifier<Events<W>> for MaxFps {
    fn modify(self, events: &mut Events<W>) {
        let MaxFps(frames) = self;
        events.dt_frame_in_ns = BILLION / frames;
    }
}

/// Defines generic constraints for event loop.
/// This is auto implemented by all types that satisfy the constraints.
pub trait EventWindow<I: GenericEvent>: PollEvent<I> + GetShouldClose + GetSize + SwapBuffers {}

impl<T: PollEvent<I> + GetShouldClose + GetSize + SwapBuffers, I: GenericEvent>
EventWindow<I> for T {}

/// A game loop iterator.
///
/// *Warning: Because the iterator polls events from the window back-end,
/// it must be used on the same thread as the window back-end (usually main thread),
/// unless the window back-end supports multi-thread event polling.*
///
/// Example:
///
/// ```Rust
/// let ref mut gl = Gl::new();
/// let window = RefCell::new(window);
/// for e in Events::new(&window)
///     .set(Ups(120))
///     .set(MaxFps(60)) {
///     use event::RenderEvent;
///     e.render(|args| {
///         // Set the viewport in window to render graphics.
///         gl.viewport(0, 0, args.width as i32, args.height as i32);
///         // Create graphics context with absolute coordinates.
///         let c = Context::abs(args.width as f64, args.height as f64);
///         // Do rendering here.
///     });
/// }
/// ```
pub struct Events<W> {
    /// The game window used by iterator.
    pub window: W,
    state: EventsState,
    last_update: u64,
    last_frame: u64,
    dt_update_in_ns: u64,
    dt_frame_in_ns: u64,
    dt: f64,
}

static BILLION: u64 = 1_000_000_000;

/// The default updates per second.
pub const DEFAULT_UPS: Ups = Ups(120);
/// The default maximum frames per second.
pub const DEFAULT_MAX_FPS: MaxFps = MaxFps(60);

impl<W: EventWindow<I>, I: GenericEvent> Events<W> {
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new(window: W) -> Events<W> {
        let start = time::precise_time_ns();
        let Ups(updates_per_second) = DEFAULT_UPS;
        let MaxFps(max_frames_per_second) = DEFAULT_MAX_FPS;
        Events {
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

impl<W: EventWindow<I>, I: GenericEvent>
Iterator<Event<I>>
for Events<W> {
    /// Returns the next game event.
    fn next(&mut self) -> Option<Event<I>> {
        loop {
            self.state = match self.state {
                RenderState => {
                    let ShouldClose(should_close) = self.window.get_should_close();
                    if should_close { return None; }

                    let start_render = time::precise_time_ns();
                    self.last_frame = start_render;

                    let Size([w, h]) = self.window.get_size();
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
