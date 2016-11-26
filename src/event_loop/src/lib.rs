//! A generic event loop for games and interactive applications

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

extern crate window;
extern crate input;
extern crate viewport;

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::cmp;
use window::Window;
use input::{ AfterRenderArgs, Event, IdleArgs, RenderArgs, UpdateArgs };

/// A trait for create event iterator from window.
pub trait Events {
    /// Creates event iterator from window.
    fn events(&self) -> WindowEvents;
}

impl<W> Events for W where W: Window {
    fn events(&self) -> WindowEvents {
        WindowEvents::new()
    }
}

/// Tells whether last emitted event was idle or not.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Idle {
    No,
    Yes
}

// Stores the update state right after sleep.
// This is used to avoid logic error when changing settings
// in the event loop followed by an update.
#[derive(Copy, Clone, Debug)]
struct UpdateState {
    dt_update_in_ns: u64,
    dt: f64,
}

#[derive(Copy, Clone, Debug)]
enum State {
    Render,
    SwapBuffers,
    UpdateLoop(Idle),
    HandleEvents(UpdateState),
    Update(UpdateState),
}

/// Methods implements for event loop settings.
pub trait EventLoop: Sized {
    /// The number of updates per second
    ///
    /// This is the fixed update rate on average over time.
    /// If the event loop lags, it will try to catch up.
    fn set_ups(&mut self, frames: u64);

    /// The number of updates per second
    ///
    /// This is the fixed update rate on average over time.
    /// If the event loop lags, it will try to catch up.
    fn ups(mut self, frames: u64) -> Self {
        self.set_ups(frames);
        self
    }

    /// The maximum number of frames per second
    ///
    /// The frame rate can be lower because the
    /// next frame is always scheduled from the previous frame.
    /// This causes the frames to "slip" over time.
    fn set_max_fps(&mut self, frames: u64);

    /// The maximum number of frames per second
    ///
    /// The frame rate can be lower because the
    /// next frame is always scheduled from the previous frame.
    /// This causes the frames to "slip" over time.
    fn max_fps(mut self, frames: u64) -> Self {
        self.set_max_fps(frames);
        self
    }

    /// Enable or disable automatic swapping of buffers.
    fn set_swap_buffers(&mut self, enable: bool);

    /// Enable or disable automatic swapping of buffers.
    fn swap_buffers(mut self, enable: bool) -> Self {
        self.set_swap_buffers(enable);
        self
    }

    /// Enable or disable benchmark mode.
    /// When enabled, it will render and update without sleep and ignore input.
    /// Used to test performance by playing through as fast as possible.
    fn set_bench_mode(&mut self, enable: bool);

    /// Enable or disable benchmark mode.
    /// When enabled, it will render and update without sleep and ignore input.
    /// Used to test performance by playing through as fast as possible.
    fn bench_mode(mut self, enable: bool) -> Self {
        self.set_bench_mode(enable);
        self
    }
}

/// An event loop iterator
///
/// *Warning: Because the iterator polls events from the window back-end,
/// it must be used on the same thread as the window back-end (usually main thread),
/// unless the window back-end supports multi-thread event polling.*
#[derive(Copy, Clone)]
pub struct WindowEvents {
    state: State,
    last_update: Instant,
    last_frame: Instant,
    dt_update_in_ns: u64,
    dt_frame_in_ns: u64,
    dt: f64,
    swap_buffers: bool,
    bench_mode: bool,
}

static BILLION: u64 = 1_000_000_000;

fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / BILLION;
    let nanos = (ns % BILLION) as u32;
    Duration::new(secs, nanos)
}

fn duration_to_secs(dur: Duration) -> f64 {
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1000_000_000.0
}

/// The default updates per second.
pub const DEFAULT_UPS: u64 = 120;
/// The default maximum frames per second.
pub const DEFAULT_MAX_FPS: u64 = 60;

impl WindowEvents
{
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new() -> WindowEvents {
        let start = Instant::now();
        let updates_per_second = DEFAULT_UPS;
        let max_frames_per_second = DEFAULT_MAX_FPS;
        WindowEvents {
            state: State::Render,
            last_update: start,
            last_frame: start,
            dt_update_in_ns: BILLION / updates_per_second,
            dt_frame_in_ns: BILLION / max_frames_per_second,
            dt: 1.0 / updates_per_second as f64,
            swap_buffers: true,
            bench_mode: false,
        }
    }

    /// Returns the next game event.
    pub fn next<W>(&mut self, window: &mut W) -> Option<Event<W::Event>>
        where W: Window
    {
        loop {
            if window.should_close() { return None; }
            self.state = match self.state {
                State::Render => {
                    // Handle input events before rendering,
                    // because window might be closed and destroy
                    // the graphics context.
                    if let Some(e) = window.poll_event() {
                        if self.bench_mode {
                            // Ignore input events in benchmark mode.
                            // This is to avoid the input events affecting
                            // the application state when benchmarking.
                            continue
                        } else {
                            return Some(Event::Input(e));
                        }
                    }
                    if window.should_close() {
                        return None;
                    }

                    if self.bench_mode {
                        // In benchmark mode, pretend FPS is perfect.
                        self.last_frame += ns_to_duration(self.dt_frame_in_ns);
                    } else {
                        // In normal mode, let the FPS slip if late.
                        self.last_frame = Instant::now();
                    }

                    let size = window.size();
                    let draw_size = window.draw_size();
                    if size.width != 0 && size.height != 0 {
                        // Swap buffers next time.
                        self.state = State::SwapBuffers;
                        return Some(Event::Render(RenderArgs {
                            // Extrapolate time forward to allow smooth motion.
                            ext_dt: duration_to_secs(self.last_frame.duration_since(self.last_update)),
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
                        window.swap_buffers();
                    }
                    self.state = State::UpdateLoop(Idle::No);
                    return Some(Event::AfterRender(AfterRenderArgs));
                }
                State::UpdateLoop(ref mut idle) => {
                    if self.bench_mode {
                        // In benchmark mode, pick the next event without sleep.
                        // Idle and input events are ignored.
                        // This is to avoid the input events affecting
                        // the application state when benchmarking.
                        let next_frame = self.last_frame + ns_to_duration(self.dt_frame_in_ns);
                        let next_update = self.last_update + ns_to_duration(self.dt_update_in_ns);
                        let next_event = cmp::min(next_frame, next_update);
                        if next_event == next_frame {
                            State::Render
                        } else {
                            State::HandleEvents(UpdateState {
                                dt_update_in_ns: self.dt_update_in_ns,
                                dt: self.dt,
                            })
                        }
                    } else {
                        let current_time = Instant::now();
                        let next_frame = self.last_frame + ns_to_duration(self.dt_frame_in_ns);
                        let next_update = self.last_update + ns_to_duration(self.dt_update_in_ns);
                        let next_event = cmp::min(next_frame, next_update);
                        if next_event > current_time {
                            if let Some(x) = window.poll_event() {
                                *idle = Idle::No;
                                return Some(Event::Input(x));
                            } else if *idle == Idle::No {
                                *idle = Idle::Yes;
                                let seconds = duration_to_secs(next_event - current_time);
                                return Some(Event::Idle(IdleArgs { dt: seconds }))
                            }
                            sleep(next_event - current_time);
                            State::UpdateLoop(Idle::No)
                        } else if next_event == next_frame {
                            State::Render
                        } else {
                            State::HandleEvents(UpdateState {
                                dt_update_in_ns: self.dt_update_in_ns,
                                dt: self.dt,
                            })
                        }
                    }
                }
                State::HandleEvents(update_state) => {
                    if self.bench_mode {
                        // Ignore input events.
                        // This is to avoid the input events affecting
                        // the application state when benchmarking.
                        match window.poll_event() {
                            None => State::Update(update_state),
                            Some(_) => State::HandleEvents(update_state),
                        }
                    } else {
                        // Handle all events before updating.
                        match window.poll_event() {
                            None => State::Update(update_state),
                            Some(x) => { return Some(Event::Input(x)); },
                        }
                    }
                }
                State::Update(update_state) => {
                    self.state = State::UpdateLoop(Idle::No);
                    // Use the update state stored right after sleep.
                    // If there are any changes in settings, these will be applied on next update.
                    self.last_update += ns_to_duration(update_state.dt_update_in_ns);
                    return Some(Event::Update(UpdateArgs{ dt: update_state.dt }));
                }
            };
        }
    }
}

impl EventLoop for WindowEvents {
    fn set_ups(&mut self, frames: u64) {
        self.dt_update_in_ns = BILLION / frames;
        self.dt = 1.0 / frames as f64;
    }

    fn set_max_fps(&mut self, frames: u64) {
        self.dt_frame_in_ns = BILLION / frames;
    }

    fn set_swap_buffers(&mut self, enable: bool) {
        self.swap_buffers = enable;
    }

    fn set_bench_mode(&mut self, enable: bool) {
        self.bench_mode = enable;
    }
}
