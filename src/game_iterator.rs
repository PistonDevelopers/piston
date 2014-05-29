use time;
use std::io::timer::sleep;
use gl;
use gl::types::GLint;

use graphics::{
    AddColor,
    Clear,
    Context,
    ColorContext,
    RelativeTransform2d,
    View,
};
use {
    AssetStore,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    Gl,
    GlData,
};
use keyboard;
use mouse;
use event;

/// Render argument.
pub struct RenderArgs<'a> {
    /// Extrapolated time in seconds, used to do smooth animation.
    pub ext_dt: f64,
    /// Rust-Graphics context.
    pub context: Context<'a>,
    /// OpenGL back-end for Rust-Graphics.
    pub gl: Gl<'a>,
}

/// Update argument.
pub struct UpdateArgs<'a> {
    /// Delta time in seconds.
    pub dt: f64,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Load arguments.
pub struct LoadArgs<'a> {
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Key press arguments.
pub struct KeyPressArgs<'a> {
    /// Keyboard key.
    pub key: keyboard::Key,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Key release arguments.
pub struct KeyReleaseArgs<'a> {
    /// Keyboard key.
    pub key: keyboard::Key,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Mouse press arguments.
pub struct MousePressArgs<'a> {
    /// Mouse button.
    pub button: mouse::Button,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Mouse release arguments.
pub struct MouseReleaseArgs<'a> {
    /// Mouse button.
    pub button: mouse::Button,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Mouse move arguments.
pub struct MouseMoveArgs<'a> {
    /// y.
    pub x: f64,
    /// x.
    pub y: f64,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Mouse relative move arguments.
pub struct MouseRelativeMoveArgs<'a> {
    /// Delta x.
    pub dx: f64,
    /// Delta y.
    pub dy: f64,
    /// Asset store.
    pub asset_store: &'a mut AssetStore,
}

/// Contains the different game events.
pub enum GameEvent<'a> {
    /// Render graphics.
    Render(RenderArgs<'a>),
    /// Update physical state of the game.
    Update(UpdateArgs<'a>),
    /// Performs tasks for loading before showing anything.
    Load(LoadArgs<'a>),
    /// Pressed a keyboard key.
    KeyPress(KeyPressArgs<'a>),
    /// Released a keyboard key.
    KeyRelease(KeyReleaseArgs<'a>),
    /// Pressed a mouse button.
    MousePress(MousePressArgs<'a>),
    /// Released a mouse button.
    MouseRelease(MouseReleaseArgs<'a>),
    /// Moved mouse cursor.
    MouseMove(MouseMoveArgs<'a>),
    /// Moved mouse relative, not bounded by cursor.
    MouseRelativeMove(MouseRelativeMoveArgs<'a>),
}

enum GameIteratorState {
    LoadState,
    RenderState,
    SwapBuffersState,
    HandleEventsState,
    MouseRelativeMoveState(f64, f64),
    UpdateState,
}

/// A game loop iterator.
pub struct GameIterator<'a, W> {
    game_window: &'a mut W,
    asset_store: &'a mut AssetStore,
    state: GameIteratorState,
    gl_data: GlData,
    context: Context<'a>,
    bg: ColorContext<'a>,
    last_update: u64,
    update_time_in_ns: u64,
    dt: f64,
    min_updates_per_frame: u64,
    min_ns_per_frame: u64,
    start_render: u64,
    updated: u64,
}

static billion: u64 = 1_000_000_000;

impl<'a, W: GameWindow> GameIterator<'a, W> {
    /// Creates a new game iterator.
    pub fn new(game_window: &'a mut W, asset_store: &'a mut AssetStore) -> GameIterator<'a, W> {
        let bg = game_window.get_settings().background_color;
        let context = Context::new();
        let bg = context.color(bg);
        let updates_per_second: u64 = 120;
        let max_frames_per_second: u64 = 60;
        // You can make this lower if needed
        let min_updates_per_frame: u64 = updates_per_second / max_frames_per_second;
        let dt: f64 = 1.0 / updates_per_second as f64;
        let update_time_in_ns: u64 = billion / updates_per_second;

        let start = time::precise_time_ns();
        let min_ns_per_frame = billion / max_frames_per_second;
        let mut last_update = start;
        GameIterator {
            game_window: game_window,
            asset_store: asset_store,
            state: LoadState,
            gl_data: GlData::new(),
            context: context,
            last_update: last_update,
            update_time_in_ns: update_time_in_ns,
            dt: dt,
            min_updates_per_frame: min_updates_per_frame,
            min_ns_per_frame: min_ns_per_frame,
            start_render: start,
            bg: bg.clone(),
            updated: 0,
        }
    }

    /// Returns the next game event.
    pub fn next<'a>(&'a mut self) -> Option<GameEvent<'a>> {
        if self.game_window.should_close() { return None; }

        match self.state {
            LoadState => {
                self.state = RenderState;
                return Some(Load(LoadArgs {
                    asset_store: self.asset_store
                }));
            },
            RenderState => {
                let start_render = time::precise_time_ns();
                // Rendering code
                let (w, h) = self.game_window.get_size();
                if w != 0 && h != 0 {
                    // Swap buffers next time.
                    self.state = SwapBuffersState;
                    gl::Viewport(0, 0, w as GLint, h as GLint);
                    // self.viewport(self.game_window);
                    let mut gl = Gl::new(&mut self.gl_data, self.asset_store);
                    self.bg.clear(&mut gl);
                    // Extrapolate time forward to allow smooth motion.
                    // 'now' is always bigger than 'last_update'.
                    let ext_dt = (start_render - self.last_update) as f64 / billion as f64;
                    return Some(Render(RenderArgs {
                        ext_dt: ext_dt, 
                        context: self.context
                            .trans(-1.0, 1.0)
                            .scale(2.0 / w as f64, -2.0 / h as f64)
                            .store_view().clone(), 
                        gl: gl
                    }));
                }

                // Jump to next state.
                // Reset update counter.
                self.updated = 0;
                self.state = UpdateState;
                return self.next();
            },
            SwapBuffersState => {
                self.game_window.swap_buffers();
                // Reset update counter.
                self.updated = 0;
                self.state = UpdateState;
                return self.next();
            },
            HandleEventsState => {
                let next_render = self.start_render + self.min_ns_per_frame;

                if // If we haven't reached the required number of updates yet
                  ( self.updated < self.min_updates_per_frame
                  // Or we have the time to update further
                  || time::precise_time_ns() < next_render )
                  // And we haven't already progressed time to far  
                  && self.last_update + self.update_time_in_ns < next_render {

                    self.last_update += self.update_time_in_ns;

                    // Handle all events before updating.
                    return match self.game_window.poll_event() {
                        event::KeyPressed(key) => {
                            Some(KeyPress(KeyPressArgs {
                                key: key,
                                asset_store: self.asset_store,
                            }))
                        },
                        event::KeyReleased(key) => {
                            Some(KeyRelease(KeyReleaseArgs {
                                key: key,
                                asset_store: self.asset_store,
                            }))
                        },
                        event::MouseButtonPressed(mouse_button) => {
                            Some(MousePress(MousePressArgs {
                                button: mouse_button,
                                asset_store: self.asset_store,
                            }))
                        },
                        event::MouseButtonReleased(mouse_button) => {
                            Some(MouseRelease(MouseReleaseArgs {
                                button: mouse_button,
                                asset_store: self.asset_store,
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
                                asset_store: self.asset_store,
                            }))
                        },
                        event::NoEvent => {
                            self.state = UpdateState;
                            self.next()
                        },
                    }
                }

                // Wait if possible
                let t = (next_render - time::precise_time_ns() ) / 1_000_000;
                if t > 1 && t < 1000000 { // The second half just checks if it overflowed,
                                          // which tells us that t should have been negative 
                                          // and we are running slow and shouldn't sleep.
                    sleep( t );
                }
                self.state = RenderState;
                return self.next();
            },
            MouseRelativeMoveState(dx, dy) => {
                self.state = HandleEventsState;
                return Some(MouseRelativeMove(MouseRelativeMoveArgs {
                    dx: dx,
                    dy: dy,
                    asset_store: self.asset_store,
                }));
            },
            UpdateState => {
                self.updated += 1;
                self.state = HandleEventsState;
                return Some(Update(UpdateArgs{
                    dt: self.dt,
                    asset_store: self.asset_store
                }));
            },
        };

        /*
        // copied.

        while !self.should_close(game_window) {

            let start_render = time::precise_time_ns();

            // Rendering code
            let (w, h) = game_window.get_size();
            if w != 0 && h != 0 {
                self.viewport(game_window);
                let mut gl = Gl::new(&mut gl_data, asset_store);
                bg.clear(&mut gl);
                // Extrapolate time forward to allow smooth motion.
                // 'now' is always bigger than 'last_update'.
                let ext_dt = (start_render - last_update) as f64 / billion as f64;
                self.render(
                    ext_dt, 
                    &context
                        .trans(-1.0, 1.0)
                        .scale(2.0 / w as f64, -2.0 / h as f64)
                        .store_view(), 
                    &mut gl
                        );
                self.swap_buffers(game_window);
            }

            let next_render = start_render + min_ns_per_frame;

            // Update gamestate
            let mut updated = 0;

            while // If we haven't reached the required number of updates yet
                  ( updated < min_updates_per_frame ||         
                    // Or we have the time to update further
                    time::precise_time_ns() < next_render ) && 
                  //And we haven't already progressed time to far  
                  last_update + update_time_in_ns < next_render {

                self.handle_events(game_window, asset_store);
                self.update(dt, asset_store);

                updated += 1;
                last_update += update_time_in_ns;
            }

            // Wait if possible

            let t = (next_render - time::precise_time_ns() ) / 1_000_000;
            if t > 1 && t < 1000000 { // The second half just checks if it overflowed,
                                      // which tells us that t should have been negative 
                                      // and we are running slow and shouldn't sleep.
                sleep( t );
            }

        }
        */
    }
}

#[test]
fn test_game_iterator() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Test".to_owned(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::empty();

    let mut game_loop = GameIterator::new(&mut window, &mut asset_store);
    loop { match game_loop.next() { None => { break }, Some(e) => {
        match e {
            Load(e) => { println!("Loading...") },
            _ => {},
        };
    } } }
}

