//! A generic event loop for games and interactive applications

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

extern crate window;
extern crate input;
extern crate viewport;
extern crate time;

use std::thread::sleep;
use std::time::Duration;
use std::cmp;
use window::Window;
use input::{AfterRenderArgs, Event, IdleArgs, RenderArgs, UpdateArgs};

/// A trait for create event iterator from window.
pub trait Events {
    /// Creates event iterator from window.
    fn events(&self) -> WindowEvents;
}

impl<W> Events for W
    where W: Window
{
    fn events(&self) -> WindowEvents {
        WindowEvents::new()
    }
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
    PrepareRender,
    PrepareUpdate(UpdateState),
    Render,
    SwapBuffers,
    UpdateLoop,
    Idle(u64),
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
    last_update: u64,
    last_frame: u64,
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

/// The default updates per second.
pub const DEFAULT_UPS: u64 = 120;
/// The default maximum frames per second.
pub const DEFAULT_MAX_FPS: u64 = 60;

impl WindowEvents {
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new() -> WindowEvents {
        let start = time::precise_time_ns();
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
            if window.should_close() {
                return None;
            }
            self.state = match self.state {
                State::PrepareRender => {
                    if self.bench_mode {
                        match window.poll_event() {
                            None => State::Render,
                            Some(_) => State::PrepareRender,
                        }
                    } else {
                        match window.poll_event() {
                            None => State::Render,
                            Some(event) => return Some(Event::Input(event)),
                        }
                    }
                }
                State::Render => {
                    if self.bench_mode {
                        // In benchmark mode, pretend FPS is perfect.
                        self.last_frame += self.dt_frame_in_ns;
                    } else {
                        // In normal mode, let the FPS slip if late.
                        self.last_frame = time::precise_time_ns();
                    }

                    let size = window.size();
                    let draw_size = window.draw_size();
                    if size.width != 0 && size.height != 0 {
                        // Swap buffers next time.
                        self.state = State::SwapBuffers;
                        return Some(Event::Render(RenderArgs {
                            // Extrapolate time forward to allow smooth motion.
                            ext_dt: (self.last_frame - self.last_update) as f64 / BILLION as f64,
                            width: size.width,
                            height: size.height,
                            draw_width: draw_size.width,
                            draw_height: draw_size.height,
                        }));
                    }

                    State::UpdateLoop
                }
                State::SwapBuffers => {
                    if self.swap_buffers {
                        window.swap_buffers();
                    }
                    self.state = State::UpdateLoop;
                    return Some(Event::AfterRender(AfterRenderArgs));
                }
                State::UpdateLoop => {
                    let current_time = time::precise_time_ns();
                    let next_frame = self.last_frame + self.dt_frame_in_ns;
                    let next_update = self.last_update + self.dt_update_in_ns;
                    let next_event = cmp::min(next_frame, next_update);

                    // In benchmark mode, pick the next event without sleep.
                    // Idle and input events are ignored.
                    if !self.bench_mode && next_event > current_time {
                        if let Some(x) = window.poll_event() {
                            return Some(Event::Input(x));
                        } else {
                            self.state = State::Idle(next_event);
                            let seconds = ((next_event - current_time) as f64) / (BILLION as f64);
                            return Some(Event::Idle(IdleArgs { dt: seconds }));
                        }
                    } else if next_event == next_frame {
                        State::PrepareRender
                    } else {
                        State::PrepareUpdate(UpdateState {
                            dt_update_in_ns: self.dt_update_in_ns,
                            dt: self.dt,
                        })
                    }
                }
                State::Idle(stop_time) => {
                    sleep(ns_to_duration(stop_time - time::precise_time_ns()));
                    State::UpdateLoop
                }
                State::PrepareUpdate(update_state) => {
                    if self.bench_mode {
                        // Ignore input to prevent it affecting the benchmark.
                        match window.poll_event() {
                            None => State::Update(update_state),
                            Some(_) => State::PrepareUpdate(update_state),
                        }
                    } else {
                        // Handle all events before updating.
                        match window.poll_event() {
                            None => State::Update(update_state),
                            Some(event) => {
                                return Some(Event::Input(event));
                            }
                        }
                    }
                }
                State::Update(update_state) => {
                    self.state = State::UpdateLoop;
                    // Use the update state stored right after sleep.
                    // If there are any changes in settings, these will be applied on next update.
                    self.last_update += update_state.dt_update_in_ns;
                    return Some(Event::Update(UpdateArgs { dt: update_state.dt }));
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
