
extern crate time;
extern crate gl;
extern crate graphics;
extern crate piston;

use self::gl::types::GLint;
use self::graphics::{
    Context,
    RelativeTransform2d,
    View,
};
use self::piston::{
    AssetStore,
    GlData,
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

pub trait EventGame {
    /// Perform tasks for loading before showing anything.
    fn load(&mut self, _asset_store: &mut AssetStore) {}

    /// Register event before game loop
    fn register_event(&mut self, _event_center: &mut EventCenter) {}

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

    fn handle_events<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        event_center: &mut EventCenter
    ) {
        loop {
            let event = game_window.poll_event();
            if event == event::NoEvent {
                break;
            }
            event_center.receive_event(&event);
        }
    }

    /// Update the physical state of the game.
    ///
    /// `dt` is the delta time from last update in seconds.
    fn update(&mut self, _dt: f64, _event_center: &mut EventCenter, _asset_store: &mut AssetStore) {}

    /// Render graphics.
    ///
    /// `context` is a Rust-Graphics context.
    /// `gl` is the Piston OpenGL back-end for Rust-Graphics.
    fn render(&self, _ext_dt: f64, _context: &Context, _gl: &mut Gl) {}


    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        use graphics::{Clear, AddColor};

        let mut event_center = EventCenter::new();

        self.load(asset_store);
        self.register_event(&mut event_center);

        let mut gl_data = GlData::new();
        let context = Context::new();
        let bg = game_window.get_settings().background_color;
        let bg = context.rgba(bg[0], bg[1], bg[2], bg[3]);
        let updates_per_second: f64 = 120.0;
        let max_updates_per_frame: u32 = 240;
        let max_frames_per_second: f64 = 60.0;

        let billion: u64 = 1_000_000_000;
        let dt: f64 = 1.0 / updates_per_second;
        let update_time_in_ns: u64 = billion / updates_per_second as u64;

        let start = time::precise_time_ns();
        let min_ns_per_frame = (billion as f64 / max_frames_per_second) as u64;
        let mut next_render = start;
        let mut last_update = start;
        while !self.should_close(game_window) {
            let now = time::precise_time_ns();

            // Render.
            let (w, h) = game_window.get_size();
            if w != 0 && h != 0 {
                self.viewport(game_window);
                let mut gl = Gl::new(&mut gl_data, asset_store);
                bg.clear(&mut gl);
                // Extrapolate time forward to allow smooth motion.
                // 'now' is always bigger than 'last_update'.
                let ext_dt = (now - last_update) as f64 / billion as f64;
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

            // Set moment for next rendering.
            next_render = now + min_ns_per_frame;
            // Perform updates by fixed time step until it catches up.
            for _ in range(0, max_updates_per_frame) {
                // Break when catching up to next frame.
                if next_render <= last_update { break; }

                // Handle user input.
                // This is handled every update to make it more responsive.
                self.handle_events(game_window, &mut event_center);

                // Update application state.
                event_center.update(dt);
                self.update(dt, &mut event_center, asset_store);
                last_update += update_time_in_ns;
            }
        }
    }
}

