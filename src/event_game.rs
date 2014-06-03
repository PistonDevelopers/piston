
extern crate time;
extern crate gl;
extern crate graphics;
extern crate piston;

use std::io::timer::sleep;
use self::gl::types::GLint;
use self::graphics::{
    Context,
    RelativeTransform2d,
    View,
};
use self::piston::{
    AssetStore,
    event,
    Game,
    GameWindow,
    Gl,
};
use {
    Event,
    EventCenter,
    EventType,
    KeyType,
};

impl KeyType for piston::keyboard::Key {
    fn id(&self) -> uint {
        // add the last enum item in piston::mouse::Button
        self.code() as uint + piston::mouse::Button8 as uint + 1
    }
}

impl KeyType for piston::mouse::Button {
    fn id(&self) -> uint {
        *self as uint
    }
}

impl EventType for piston::event::Event {
    fn is_press_key(&self, key: &KeyType) -> bool {
        match *self {
            piston::event::KeyPressed(k) if k.id() == key.id() => {
                true
            },
            piston::event::MouseButtonPressed(k) if k.id() == key.id() => {
                true
            },
            _ => {
                false
            },
        }
    }
    fn is_release_key(&self, key: &KeyType) -> bool {
        match *self {
            piston::event::KeyReleased(k) if k.id() == key.id() => {
                true
            },
            piston::event::MouseButtonReleased(k) if k.id() == key.id() => {
                true
            },
            _ => {
                false
            }
        }
    }
}

/// ******************************
/// * MOST CODE COPY FROM PISTON *
/// ******************************
///
/// Implemented by game application which want to use rust-event.
pub trait EventGame {
    /// Perform tasks for loading before showing anything.
    fn load(&mut self, _asset_store: &mut AssetStore) {}

    /// Register event before game loop
    fn register_event(&mut self, _event_center: &mut EventCenter<Self>) {}

    /// Sets up viewport.
    ///
    /// A viewport is the region of the window where graphics is rendered.
    #[inline(always)]
    fn viewport<W: GameWindow>(&self, game_window: &W) {
        let (w, h) = game_window.get_size();
        gl::Viewport(0, 0, w as GLint, h as GLint);
    }

    /// Whether the window should be closed.
    ///
    /// When this is `true` the application shuts down.
    /// This can be overridden to emulate a user closing the window.
    /// One can also override this method to prevent window from closing.
    fn should_close<W: GameWindow>(&self, game_window: &W) -> bool {
        game_window.should_close()
    }

    /// Swaps the front buffer with the back buffer.
    ///
    /// When called, This shows the next frame.
    /// The graphics is rendered to the back buffer.
    /// The front buffer is displayed on the screen.
    fn swap_buffers<W: GameWindow>(&self, game_window: &W) {
        game_window.swap_buffers()
    }

    /// Handle current window's event with EventCenter.
    fn handle_events<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        event_center: &mut EventCenter<Self>
    ) {
        loop {
            let event = game_window.poll_event();
            match event {
                event::NoEvent => {
                    break;
                },
                _ => {
                    event_center.receive_event(&event);
                },
            }
        }
    }

    /// Update the physical state of the game.
    ///
    /// `dt` is the delta time from last update in seconds.
    fn update(&mut self, _dt: f64, _event_center: &mut EventCenter<Self>, _asset_store: &mut AssetStore) {}

    /// Render graphics.
    ///
    /// `context` is a Rust-Graphics context.
    /// `gl` is the Piston OpenGL back-end for Rust-Graphics.
    fn render(&self, _ext_dt: f64, _context: &Context, _gl: &mut Gl) {}

    /// Executes a game loop.
    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        use graphics::{Clear, AddColor};

        let mut event_center = EventCenter::new();

        self.load(asset_store);
        self.register_event(&mut event_center);

        let mut gl = Gl::new();
        let context = Context::new();
        let bg = game_window.get_settings().background_color;
        let bg = context.rgba(bg[0], bg[1], bg[2], bg[3]);

        let billion: u64 = 1_000_000_000;
        let updates_per_second = 120.0;
        let dt: f64 = 1.0 / updates_per_second;
        let update_time_in_ns: u64 = billion / updates_per_second as u64;

        let max_frames_per_second: f64 = 60.0;
        let min_ns_per_frame = (billion as f64 / max_frames_per_second) as u64;

        let mut last_update = time::precise_time_ns();
        let mut lag = 0;
        while !self.should_close(game_window) {
            let now = time::precise_time_ns();
            let elapsed = now - last_update;
            last_update = now;
            lag += elapsed;

            // Perform updates by fixed time step until it catches up.
            while lag >= update_time_in_ns {
                // Handle user input.
                // This is handled every update to make it more responsive.
                self.handle_events(game_window, &mut event_center);

                // Update application state.
                event_center.update(self, dt);
                self.update(dt, &mut event_center, asset_store);
                lag -= update_time_in_ns;
            }

            // Render.
            let (w, h) = game_window.get_size();
            if w != 0 && h != 0 {
                self.viewport(game_window);
                bg.clear(&mut gl);
                // Extrapolate time forward to allow smooth motion.
                // 'lag' is always less than 'update_time_in_ns'.
                let ext_dt = lag as f64 / update_time_in_ns as f64;
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

            let used_frame_time = time::precise_time_ns() - now;
            // sleep at least 1 ms
            if min_ns_per_frame > used_frame_time + 1_000_000  {
                sleep((min_ns_per_frame - used_frame_time) / 1_000_000);
            }
        }
    }
}

