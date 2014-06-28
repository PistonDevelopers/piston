//! FIXME! This is a lot of code duplication

use time;
use std::io::timer::sleep;

use GameLoopWindow;
use event;

use game_iterator::{
    RenderArgs,
    UpdateArgs,
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,

    GameEvent,
    GameIteratorSettings,

    Render,
    KeyPress,
    KeyRelease,
    MousePress,
    MouseRelease,
    MouseMove,
    MouseRelativeMove,
    Update,
};

//use game_iterator::GameEvent::*;


enum ConcurrentIteratorState {
    RenderState,
    PrepareUpdateLoopState,
    UpdateLoopState,
    HandleEventsState,
    MouseRelativeMoveState(f64, f64),
    UpdateState,
}

/// As GameIterator, except without swapping buffers
/// For use with concurrent updating and rendering.
pub struct ConcurrentIterator<'a, W> {
    game_window: &'a mut W,
    state: ConcurrentIteratorState,
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

impl<'a, W: GameLoopWindow> ConcurrentIterator<'a, W> {
    /// Creates a new concurrent iterator.
    pub fn new(game_window: &'a mut W, settings: &GameIteratorSettings) -> ConcurrentIterator<'a, W> {
        let updates_per_second: u64 = settings.updates_per_second;
        let max_frames_per_second: u64 = settings.max_frames_per_second;

        let start = time::precise_time_ns();
        ConcurrentIterator {
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

    /// Returns the next game event.
    pub fn next<'a>(&'a mut self) -> Option<GameEvent<'a>> {
       match self.state {
            RenderState => {
                if self.game_window.should_close() { return None; }

                self.start_render = time::precise_time_ns();
                // Rendering code
                let (w, h) = self.game_window.get_size();
                if w != 0 && h != 0 {
                    self.state = PrepareUpdateLoopState;
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
    }
}
