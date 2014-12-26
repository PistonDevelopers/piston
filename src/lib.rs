//! A generic event loop for games and interactive applications

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

extern crate time;
extern crate current;

use std::io::timer::sleep;
use std::time::duration::Duration;
use current::{ ActOn, Action, GetFrom, Get, SetAt };
use std::cmp;

/// Whether window should close or not.
#[deriving(Copy)]
pub struct ShouldClose(pub bool);

impl Sized for ShouldClose {}

/// The size of the window.
#[deriving(Copy)]
pub struct Size(pub [u32, ..2]);

impl Sized for Size {}

/// Tells window to swap buffers.
///
/// ~~~ignore
/// use current::Action;
///
/// ...
/// window.action(SwapBuffers);
/// ~~~
#[deriving(Copy)]
pub struct SwapBuffers;

impl Sized for SwapBuffers {}

/// Polls event from window.
///
/// ~~~ignore
/// use current::Action;
///
/// ...
/// let e = window.action(PollEvent);
/// ~~~
#[deriving(Copy)]
pub struct PollEvent;

impl Sized for PollEvent {}

/// Render arguments
#[deriving(Copy, Clone, PartialEq, Show)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area.
    pub width: u32,
    /// The height of rendered area.
    pub height: u32,
}

/// Update arguments, such as delta time in seconds
#[deriving(Copy, Clone, PartialEq, Show)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// Methods required to map from consumed event to emitted event.
pub trait EventMap<I> {
    /// Creates a render event.
    fn render(args: RenderArgs) -> Self;
    /// Creates an update event.
    fn update(args: UpdateArgs) -> Self;
    /// Creates an input event.
    fn input(args: I) -> Self;
}

#[deriving(Copy, Show)]
enum State {
    Render,
    SwapBuffers,
    UpdateLoop,
    HandleEvents,
    Update,
}

/// The number of updates per second
///
/// This is the fixed update rate on average over time.
/// If the event loop lags, it will try to catch up.
#[deriving(Copy)]
pub struct Ups(pub u64);

impl<W> SetAt<Events<W>> for Ups {
    fn set_at(self, events: &mut Events<W>) {
        let Ups(frames) = self;
        events.dt_update_in_ns = BILLION / frames;
        events.dt = 1.0 / frames as f64;
    }
}

/// The maximum number of frames per second
///
/// The frame rate can be lower because the
/// next frame is always scheduled from the previous frame.
/// This causes the frames to "slip" over time.
#[deriving(Copy)]
pub struct MaxFps(pub u64);

impl<W> SetAt<Events<W>> for MaxFps {
    fn set_at(self, events: &mut Events<W>) {
        let MaxFps(frames) = self;
        events.dt_frame_in_ns = BILLION / frames;
    }
}

/// An event loop iterator
///
/// *Warning: Because the iterator polls events from the window back-end,
/// it must be used on the same thread as the window back-end (usually main thread),
/// unless the window back-end supports multi-thread event polling.*
///
/// Example:
///
/// ~~~ignore
/// fn main() {
///     let opengl = shader_version::opengl::OpenGL_3_2;
///     let window = Sdl2Window::new(
///         opengl,
///         WindowSettings {
///             title: "Example".to_string(),
///             size: [500, 500],
///             fullscreen: false,
///             exit_on_esc: true,
///             samples: 0,
///         }
///     )
///     let ref mut gl = Gl::new();
///     let window = RefCell::new(window);
///     for e in Events::new(&window)
///         .set(Ups(120))
///         .set(MaxFps(60)) {
///         use event::RenderEvent;
///         e.render(|args| {
///             // Set the viewport in window to render graphics.
///             gl.viewport(0, 0, args.width as i32, args.height as i32);
///             // Create graphics context with absolute coordinates.
///             let c = Context::abs(args.width as f64, args.height as f64);
///             // Do rendering here.
///         });
///     }
/// }
/// ~~~
pub struct Events<W> {
    /// The game window used by iterator.
    pub window: W,
    state: State,
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

impl<W> Events<W> {
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new(window: W) -> Events<W> {
        let start = time::precise_time_ns();
        let Ups(updates_per_second) = DEFAULT_UPS;
        let MaxFps(max_frames_per_second) = DEFAULT_MAX_FPS;
        Events {
            window: window,
            state: State::Render,
            last_update: start,
            last_frame: start,
            dt_update_in_ns: BILLION / updates_per_second,
            dt_frame_in_ns: BILLION / max_frames_per_second,
            dt: 1.0 / updates_per_second as f64,
        }
    }
}

impl<W, I, E: EventMap<I>>
Iterator<E>
for Events<W>
    where
        ShouldClose: GetFrom<W>,
        Size: GetFrom<W>,
        SwapBuffers: ActOn<W, ()>,
        PollEvent: ActOn<W, Option<I>>
{
    /// Returns the next game event.
    fn next(&mut self) -> Option<E> {
        loop {
            self.state = match self.state {
                State::Render => {
                    let ShouldClose(should_close) = self.window.get();
                    if should_close { return None; }

                    let start_render = time::precise_time_ns();
                    self.last_frame = start_render;

                    let Size([w, h]) = self.window.get();
                    if w != 0 && h != 0 {
                        // Swap buffers next time.
                        self.state = State::SwapBuffers;
                        return Some(EventMap::render(RenderArgs {
                            // Extrapolate time forward to allow smooth motion.
                            ext_dt: (start_render - self.last_update) as f64
                                    / BILLION as f64,
                            width: w,
                            height: h,
                        }));
                    }

                    State::UpdateLoop
                }
                State::SwapBuffers => {
                    self.window.action(SwapBuffers);
                    State::UpdateLoop
                }
                State::UpdateLoop => {
                    let current_time = time::precise_time_ns();
                    let next_frame = self.last_frame + self.dt_frame_in_ns;
                    let next_update = self.last_update + self.dt_update_in_ns;
                    let next_event = cmp::min(next_frame, next_update);
                    if next_event > current_time {
                        sleep( Duration::nanoseconds((next_event - current_time) as i64) );
                        State::UpdateLoop
                    } else if next_event == next_frame {
                        State::Render
                    } else {
                        State::HandleEvents
                    }
                }
                State::HandleEvents => {
                    // Handle all events before updating.
                    match self.window.action(PollEvent) {
                        None => State::Update,
                        Some(x) => { return Some(EventMap::input(x)); },
                    }
                }
                State::Update => {
                    self.state = State::UpdateLoop;
                    self.last_update += self.dt_update_in_ns;
                    return Some(EventMap::update(UpdateArgs{ dt: self.dt }));
                }
            };
        }
    }
}
