//! Game loop.

// External crates.
use time;
use gl = opengles::gl2;
use graphics::*;

// Local crate.
use Gl = gl::Gl;
use GlData = gl::GlData;
use GameWindow = game_window::GameWindow;
use AssetStore = asset_store::AssetStore;

use game_window::{
    keycode,
    event,
};

/// Implemented by game applications.
pub trait Game<W: GameWindow> {
    /// Render graphics.
    ///
    /// `context` is a Rust-Graphics context.
    /// `gl` is the Piston OpenGL back-end for Rust-Graphics.
    fn render(&self, _context: &Context, _gl: &mut Gl) {}

    /// Update the physical state of the game.
    ///
    /// `dt` is the delta time from last update in seconds.
    fn update(&mut self, _dt: f64, _asset_store: &mut AssetStore) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self, _asset_store: &mut AssetStore) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(
        &mut self,
        _key: keycode::KeyCode,
        _asset_store: &mut AssetStore
    ) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(
        &mut self,
        _key: keycode::KeyCode,
        _asset_store: &mut AssetStore
    ) {}

    /// Sets up viewport.
    ///
    /// A viewport is the region of the window where graphics is rendered.
    #[inline(always)]
    fn viewport(&self, game_window: &W) {
        let (w, h) = game_window.get_size();
        gl::viewport(0, 0, w as gl::GLint, h as gl::GLint);
    }

    /// Whether the window should be closed.
    ///
    /// When this is `true` the application shuts down.
    /// This can be overridden to emulate a user closing the window.
    /// One can also override this method to prevent window from closing.
    fn should_close(&self, game_window: &W) -> bool {
        game_window.should_close()
    }

    /// Swaps the front buffer with the back buffer.
    ///
    /// When called, This shows the next frame.
    /// The graphics is rendered to the back buffer.
    /// The front buffer is displayed on the screen.
    fn swap_buffers(&self, game_window: &W) {
        use glfw::Context;

        game_window.swap_buffers()
    }

    /// Handles events using current game window settings.
    ///
    /// This can be overriden to do custom event handling.
    fn handle_events(
        &mut self,
        game_window: &W,
        asset_store: &mut AssetStore
    ) {
        /*let exit_on_esc = game_window.settings.exit_on_esc;
        game_window.glfw.poll_events();
        for (_, event) in
        glfw::flush_messages(&game_window.events) {
            match event {
                // Close with Esc.
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _)
                if exit_on_esc  => {
                    game_window.set_should_close(true)
                },
                glfw::KeyEvent(key, _, glfw::Press, _) => {
                    self.key_press(key, asset_store)
                },
                glfw::KeyEvent(key, _, glfw::Release, _) => {
                    self.key_release(key, asset_store)
                },
                _ => {},
            }
        }*/
        loop {
            match game_window.poll_event() {
                event::KeyDownEvent(keycode) => {
                    self.key_press(keycode, asset_store)
                },
                event::KeyUpEvent(keycode) => {
                    self.key_release(keycode, asset_store)
                },
                event::NoEvent => {
                    break
                },
            }
        }
    }

    /// Executes a game loop.
    ///
    /// The loop continues until `should_close` returns true.
    fn run(
        &mut self,
        game_window: &W,
        asset_store: &mut AssetStore
    ) {
        use graphics::{Clear, AddColor};
        use gl::Gl;

        self.load(asset_store);
        let mut gl_data = GlData::new();
        let context = Context::new();
        let bg = game_window.get_settings().background_color;
        let bg = context.rgba(bg[0], bg[1], bg[2], bg[3]);
        let updates_per_second: u64 = 100;
        let dt: f64 = 1.0 / updates_per_second as f64;
        let update_time_in_ns: u64 = 1_000_000_000 / updates_per_second;
        let mut last_update = time::precise_time_ns();
        while !self.should_close(game_window) {
            self.viewport(game_window);
            let (w, h) = game_window.get_size();
            if w != 0 && h != 0 {
                let mut gl = Gl::new(&mut gl_data, asset_store);
                bg.clear(&mut gl);
                self.render(&context
                .trans_local(-1.0, 1.0)
                .scale_local(2.0 / w as f64, -2.0 / h as f64)
                .store_view()
                .reset(), &mut gl);
            }
            self.swap_buffers(game_window);
            // Perform updates by fixed time step until it catches up.
            loop {
                self.update(dt, asset_store);
                last_update += update_time_in_ns;
                let now = time::precise_time_ns();
                if now <= last_update { break; }
            }
            self.handle_events(game_window, asset_store);
        }
    }
}

