//! A generic event loop for games and interactive applications

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

extern crate clock_ticks;
extern crate window;
extern crate viewport;

use std::thread::sleep_ms;
use std::cmp;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;
use window::Window;
use viewport::Viewport;

/// Render arguments
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RenderArgs {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// The width of rendered area in points.
    pub width: u32,
    /// The height of rendered area in points.
    pub height: u32,
    /// The width of rendered area in pixels.
    pub draw_width: u32,
    /// The height of rendered area in pixels.
    pub draw_height: u32,
}

impl RenderArgs {
    /// Returns viewport information filling entire render area.
    pub fn viewport(&self) -> Viewport {
        Viewport {
            rect: [0, 0, self.draw_width as i32, self.draw_height as i32],
            window_size: [self.width, self.height],
            draw_size: [self.draw_width, self.draw_height],
        }
    }
}

/// After render arguments.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AfterRenderArgs;

/// Update arguments, such as delta time in seconds
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// Idle arguments, such as expected idle time in seconds.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IdleArgs {
    /// Expected idle time in seconds.
    pub dt: f64
}

/// Methods required to map from consumed event to emitted event.
pub trait EventMap<I> {
    /// Creates a render event.
    fn render(args: RenderArgs) -> Self;
    /// Creates an after render event.
    fn after_render(args: AfterRenderArgs) -> Self;
    /// Creates an update event.
    fn update(args: UpdateArgs) -> Self;
    /// Creates an input event.
    fn input(args: I) -> Self;
    /// Creates an idle event.
    fn idle(IdleArgs) -> Self;
}

/// Tells whether last emitted event was idle or not.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Idle {
    No,
    Yes
}

#[derive(Copy, Clone, Debug)]
enum State {
    Render,
    SwapBuffers,
    UpdateLoop(Idle),
    HandleEvents,
    Update,
}

/// An event loop iterator
///
/// *Warning: Because the iterator polls events from the window back-end,
/// it must be used on the same thread as the window back-end (usually main thread),
/// unless the window back-end supports multi-thread event polling.*
pub struct WindowEvents<W, E>
    where
        W: Window,
        E: EventMap<<W as Window>::Event>
{
    window: Rc<RefCell<W>>,
    state: State,
    last_update: u64,
    last_frame: u64,
    dt_update_in_ns: u64,
    dt_frame_in_ns: u64,
    dt: f64,
    swap_buffers: bool,
    _marker_e: PhantomData<E>,
}

static BILLION: u64 = 1_000_000_000;

fn ns_to_ms(ns: u64) -> u32 {
    (ns / 1_000_000) as u32
}

/// The default updates per second.
pub const DEFAULT_UPS: u64 = 120;
/// The default maximum frames per second.
pub const DEFAULT_MAX_FPS: u64 = 60;

impl<W, E> WindowEvents<W, E>
    where
        W: Window,
        E: EventMap<<W as Window>::Event>
{
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new(window: Rc<RefCell<W>>) -> WindowEvents<W, E> {
        let start = clock_ticks::precise_time_ns();
        let updates_per_second = DEFAULT_UPS;
        let max_frames_per_second = DEFAULT_MAX_FPS;
        WindowEvents {
            window: window,
            state: State::Render,
            last_update: start,
            last_frame: start,
            dt_update_in_ns: BILLION / updates_per_second,
            dt_frame_in_ns: BILLION / max_frames_per_second,
            dt: 1.0 / updates_per_second as f64,
            swap_buffers: true,
            _marker_e: PhantomData,
        }
    }

    /// The number of updates per second
    ///
    /// This is the fixed update rate on average over time.
    /// If the event loop lags, it will try to catch up.
    pub fn ups(mut self, frames: u64) -> Self {
        self.dt_update_in_ns = BILLION / frames;
        self.dt = 1.0 / frames as f64;
        self
    }

    /// The maximum number of frames per second
    ///
    /// The frame rate can be lower because the
    /// next frame is always scheduled from the previous frame.
    /// This causes the frames to "slip" over time.
    pub fn max_fps(mut self, frames: u64) -> Self {
        self.dt_frame_in_ns = BILLION / frames;
        self
    }

    /// Enable or disable automatic swapping of buffers.
    pub fn swap_buffers(mut self, enable: bool) -> Self {
        self.swap_buffers = enable;
        self
    }
}

impl<W, E> Iterator for WindowEvents<W, E>
    where
        W: Window,
        E: EventMap<<W as Window>::Event>,
{
    type Item = E;

    /// Returns the next game event.
    fn next(&mut self) -> Option<E> {
        loop {
            self.state = match self.state {
                State::Render => {
                    let window = self.window.borrow();
                    if window.should_close() { return None; }

                    let start_render = clock_ticks::precise_time_ns();
                    self.last_frame = start_render;

                    let size = window.size();
                    let draw_size = window.draw_size();
                    if size.width != 0 && size.height != 0 {
                        // Swap buffers next time.
                        self.state = State::SwapBuffers;
                        return Some(EventMap::render(RenderArgs {
                            // Extrapolate time forward to allow smooth motion.
                            ext_dt: (start_render - self.last_update) as f64
                                    / BILLION as f64,
                            width: size.width,
                            height: size.height,
                            draw_width: draw_size.width,
                            draw_height: draw_size.height,
                        }));
                    }

                    State::UpdateLoop(Idle::No)
                }
                State::SwapBuffers => {
                    if self.swap_buffers {
                        self.window.borrow_mut().swap_buffers();
                        self.state = State::UpdateLoop(Idle::No);
                        return Some(EventMap::after_render(AfterRenderArgs));
                    } else {
                        State::UpdateLoop(Idle::No)
                    }
                }
                State::UpdateLoop(ref mut idle) => {
                    let current_time = clock_ticks::precise_time_ns();
                    let next_frame = self.last_frame + self.dt_frame_in_ns;
                    let next_update = self.last_update + self.dt_update_in_ns;
                    let next_event = cmp::min(next_frame, next_update);
                    if next_event > current_time {
                        if let Some(x) = self.window.borrow_mut().poll_event() {
                            *idle = Idle::No;
                            return Some(EventMap::input(x));
                        } else if *idle == Idle::No {
                            *idle = Idle::Yes;
                            let seconds = ((next_event - current_time) as f64) / (BILLION as f64);
                            return Some(EventMap::idle(IdleArgs { dt: seconds }))
                        }
                        sleep_ms(ns_to_ms(next_event - current_time));
                        State::UpdateLoop(Idle::No)
                    } else if next_event == next_frame {
                        State::Render
                    } else {
                        State::HandleEvents
                    }
                }
                State::HandleEvents => {
                    // Handle all events before updating.
                    match self.window.borrow_mut().poll_event() {
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
