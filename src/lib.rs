//! A generic event loop for games and interactive applications

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![allow(unstable)]

extern crate clock_ticks;
extern crate quack;

use std::io::timer::sleep;
use std::time::duration::Duration;
use quack::{ ActOn, Action, GetFrom, Get, Pair, SetAt };
use std::cmp;

/// Whether window should close or not.
#[derive(Copy)]
pub struct ShouldClose(pub bool);

impl Sized for ShouldClose {}

/// The size of the window.
#[derive(Copy)]
pub struct Size(pub [u32; 2]);

impl Sized for Size {}

/// Tells window to swap buffers.
///
/// ~~~ignore
/// use current::Action;
///
/// ...
/// window.action(SwapBuffers);
/// ~~~
#[derive(Copy)]
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
#[derive(Copy)]
pub struct PollEvent;

impl Sized for PollEvent {}

/// Render arguments
#[derive(Copy, Clone, PartialEq, Show)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area.
    pub width: u32,
    /// The height of rendered area.
    pub height: u32,
}

/// Update arguments, such as delta time in seconds
#[derive(Copy, Clone, PartialEq, Show)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// Idle arguments, such as expected idle time in seconds.
#[derive(Copy, Clone, PartialEq, Show)]
pub struct IdleArgs {
    /// Expected idle time in seconds.
    pub dt: f64
}

/// Methods required to map from consumed event to emitted event.
pub trait EventMap<I> {
    /// Creates a render event.
    fn render(args: RenderArgs) -> Self;
    /// Creates an update event.
    fn update(args: UpdateArgs) -> Self;
    /// Creates an input event.
    fn input(args: I) -> Self;
    /// Creates an idle event.
    fn idle(IdleArgs) -> Self;
}

/// Tells whether last emitted event was idle or not.
#[derive(Copy, Show, PartialEq, Eq)]
enum Idle {
    No,
    Yes
}

#[derive(Copy, Show)]
enum State {
    Render,
    SwapBuffers,
    UpdateLoop(Idle),
    HandleEvents,
    Update,
}

/// The number of updates per second
///
/// This is the fixed update rate on average over time.
/// If the event loop lags, it will try to catch up.
#[derive(Copy)]
pub struct Ups(pub u64);

impl<W, I, E> SetAt for (Ups, Events<W, I, E>) {
    fn set_at(Ups(frames): Ups, events: &mut Events<W, I, E>) {
        events.dt_update_in_ns = BILLION / frames;
        events.dt = 1.0 / frames as f64;
    }
}

/// The maximum number of frames per second
///
/// The frame rate can be lower because the
/// next frame is always scheduled from the previous frame.
/// This causes the frames to "slip" over time.
#[derive(Copy)]
pub struct MaxFps(pub u64);

impl<W, I, E> SetAt for (MaxFps, Events<W, I, E>) {
    fn set_at(MaxFps(frames): MaxFps, events: &mut Events<W, I, E>) {
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
pub struct Events<W, I, E> {
    window: W,
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

impl<W, I, E> Events<W, I, E> {
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new(window: W) -> Events<W, I, E> {
        let start = clock_ticks::precise_time_ns();
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

impl<W, I, E>
Iterator
for Events<W, I, E>
    where
        (ShouldClose, W): Pair<Data = ShouldClose, Object = W> + GetFrom,
        (Size, W): Pair<Data = Size, Object = W> + GetFrom,
        (SwapBuffers, W): Pair<Data = SwapBuffers, Object = W> + ActOn<()>,
        (PollEvent, W): Pair<Data = PollEvent, Object = W> + ActOn<Option<I>>,
        E: EventMap<I>,
{
    type Item = E;

    /// Returns the next game event.
    fn next(&mut self) -> Option<E> {
        loop {
            self.state = match self.state {
                State::Render => {
                    let ShouldClose(should_close) = self.window.get();
                    if should_close { return None; }

                    let start_render = clock_ticks::precise_time_ns();
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

                    State::UpdateLoop(Idle::No)
                }
                State::SwapBuffers => {
                    self.window.action(SwapBuffers);
                    State::UpdateLoop(Idle::No)
                }
                State::UpdateLoop(ref mut idle) => {
                    let current_time = clock_ticks::precise_time_ns();
                    let next_frame = self.last_frame + self.dt_frame_in_ns;
                    let next_update = self.last_update + self.dt_update_in_ns;
                    let next_event = cmp::min(next_frame, next_update);
                    if next_event > current_time {
                        if let Some(x) = self.window.action(PollEvent) {
                            *idle = Idle::No;
                            return Some(EventMap::input(x));
                        } else if *idle == Idle::No {
                            *idle = Idle::Yes;
                            let seconds = ((next_event - current_time) as f64) / (BILLION as f64);
                            return Some(EventMap::idle(IdleArgs { dt: seconds }))
                        }
                        sleep( Duration::nanoseconds((next_event - current_time) as i64) );
                        State::UpdateLoop(Idle::No)
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
                    self.state = State::UpdateLoop(Idle::No);
                    self.last_update += self.dt_update_in_ns;
                    return Some(EventMap::update(UpdateArgs{ dt: self.dt }));
                }
            };
        }
    }
}
