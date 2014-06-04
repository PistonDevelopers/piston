//! Game loop.

// External crates.
use graphics::*;
use gl;
use gl::types::GLint;

// Local crate.
use Gl = gl_back_end::Gl;
use GameWindow = game_window::GameWindow;
use GameIteratorSettings;
use AssetStore;
use GameIterator;
use KeyPress;
use KeyRelease;
use MouseMove;
use MouseRelativeMove;
use MousePress;
use MouseRelease;
use Render;
use Update;
use keyboard;
use mouse;

/// Implemented by game applications.
pub trait Game {
    /// Render graphics.
    ///
    /// `context` is a Rust-Graphics context.
    /// `gl` is the Piston OpenGL back-end for Rust-Graphics.
    fn render(&self, _ext_dt: f64, _context: &Context, _gl: &mut Gl) {}

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

    /// Executes a game loop.
    ///
    /// The loop continues until `should_close` returns true.
    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        self.load(asset_store);

        let mut game_iter = GameIterator::new(
            game_window,
            &GameIteratorSettings {
                updates_per_second: 120,
                max_frames_per_second: 60
            });
        loop {
            match game_iter.next() {
                None => { break }
                Some(e) => match e {

Render(args) => self.render(
    args.ext_dt,
    &Context::abs(args.width as f64, args.height as f64),
    args.gl
),
Update(args) => self.update(args.dt, asset_store),
KeyPress(args) => self.key_press(args.key, asset_store),
KeyRelease(args) => self.key_release(args.key, asset_store),
MousePress(args) => self.mouse_press(args.button, asset_store),
MouseRelease(args) => self.mouse_release(args.button, asset_store),
MouseMove(args) => self.mouse_move(args.x, args.y, asset_store),
MouseRelativeMove(args) => self.mouse_relative_move(
    args.dx, 
    args.dy, 
    asset_store
),

                }
            }
        }
    }
}

