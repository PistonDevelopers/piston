use time;
use std::io::timer::sleep;
use std::time::duration::Duration;
use std::cell::RefCell;
use current::{ Get, Modifier };
use window::{
    PollEvent, 
    ShouldClose,
    Size,
    SwapBuffers,
    Window,
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

impl<'a, E> Modifier<Events<'a, E>> for Ups {
    fn modify(self, events: &mut Events<'a, E>) {
        let Ups(frames) = self;
        events.dt_update_in_ns = BILLION / frames;
    }
}

/// The maximum number of frames per second.
/// Next frame is always scheduled from the previous frame.
pub struct MaxFps(pub u64);

impl<'a, E> Modifier<Events<'a, E>> for MaxFps {
    fn modify(self, events: &mut Events<'a, E>) {
        let MaxFps(frames) = self;
        events.dt_frame_in_ns = BILLION / frames;
    }
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
pub struct Events<'a, W: 'a> {
    /// The game window used by iterator.
    pub window: &'a RefCell<W>,
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

impl<'a, W> Events<'a, W> {
    /// Creates a new event iterator with default UPS and FPS settings.
    /// Uses a `RefCell` reference to the window,
    /// because it is likely to be access elsewhere while polling events.
    pub fn new(window: &'a RefCell<W>) -> Events<'a, W> {
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

/// Wrapper for `Get<ShouldClose>`
trait GetShouldClose: Get<ShouldClose> {
    /// wraps method.
    fn get_should_close(&self) -> ShouldClose { self.get() }
}

impl<T: Get<ShouldClose>> GetShouldClose for T {}

/// Wrapper for `Get<Size>`
trait GetSize: Get<Size> {
    /// wraps method.
    fn get_size(&self) -> Size { self.get() }
}

impl<T: Get<Size>> GetSize for T {}

impl<'a, W: PollEvent<I> + GetShouldClose + GetSize
          + SwapBuffers, I: GenericEvent>
Iterator<Event<I>>
for Events<'a, W> {
    /// Returns the next game event.
    fn next(&mut self) -> Option<Event<I>> {
        let mut window = self.window.borrow_mut();
        let window = window.deref_mut();
        loop {
            self.state = match self.state {
                RenderState => {
                    let ShouldClose(should_close) = (*window).get_should_close();
                    if should_close { return None; }

                    let start_render = time::precise_time_ns();
                    self.last_frame = start_render;

                    let Size([w, h]) = (*window).get_size();
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
                    window.swap_buffers();
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
                    match window.poll_event() {
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
