//! Game loop.

// External crates.
use graphics::*;
use gl;
use gl::types::GLint;
use time;

// Local crate.
use Gl = gl_back_end::Gl;
use GlData = gl_back_end::GlData;
use GameWindow = game_window::GameWindow;
use AssetStore = asset_store::AssetStore;
use keyboard;
use event;
use mouse;

/// Implemented by game applications.
pub trait Game {
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
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(
        &mut self,
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}

    /// Pressed a mouse button.
    fn mouse_press(
        &mut self,
        _button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {}

    /// Released a mouse button.
    fn mouse_release(
        &mut self,
        _button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {}

    /// Moved mouse cursor.
    fn mouse_move(
        &mut self,
        _x: f64,
        _y: f64,
        _asset_store: &mut AssetStore
    ) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(
        &mut self,
        _dx: f64,
        _dy: f64,
        _asset_store: &mut AssetStore
    ) {}

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

    /// Handles events using current game window settings.
    ///
    /// This can be overriden to do custom event handling.
    fn handle_events<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        loop {
            match game_window.poll_event() {
                event::KeyPressed(keycode) => {
                    self.key_press(keycode, asset_store)
                },
                event::KeyReleased(keycode) => {
                    self.key_release(keycode, asset_store)
                },
                event::MouseButtonPressed(mouse_button) => {
                    self.mouse_press(mouse_button, asset_store)
                },
                event::MouseButtonReleased(mouse_button) => {
                    self.mouse_release(mouse_button, asset_store)
                },
                event::MouseMoved(x, y, relative_move) => {
                    self.mouse_move(x, y, asset_store);
                    match relative_move {
                        Some((dx, dy)) =>
                            self.mouse_relative_move(dx, dy, asset_store),
                        None => {},
                    }
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
    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        use graphics::{Clear, AddColor};

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
                .trans(-1.0, 1.0)
                .scale(2.0 / w as f64, -2.0 / h as f64)
                .store_view(), &mut gl);
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

