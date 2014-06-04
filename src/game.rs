//! Game loop.

// External crates.
use graphics::*;
use gl;
use gl::types::GLint;

// Local crate.
use GameWindow = game_window::GameWindow;
use GameIteratorSettings;
use AssetStore;
use GameIterator;
use KeyPress;
use KeyPressArgs;
use KeyRelease;
use KeyReleaseArgs;
use MouseMove;
use MouseMoveArgs;
use MouseRelativeMove;
use MouseRelativeMoveArgs;
use MousePress;
use MousePressArgs;
use MouseRelease;
use MouseReleaseArgs;
use Render;
use RenderArgs;
use Update;
use UpdateArgs;

/// Implemented by game applications.
pub trait Game {
    /// Render graphics.
    ///
    /// `context` is a Rust-Graphics context.
    /// `gl` is the Piston OpenGL back-end for Rust-Graphics.
    fn render(&self, _context: &Context, _args: RenderArgs) {}

    /// Update the physical state of the game.
    ///
    /// `dt` is the delta time from last update in seconds.
    fn update(&mut self, _asset_store: &mut AssetStore, _args: UpdateArgs) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self, _asset_store: &mut AssetStore) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(
        &mut self,
        _asset_store: &mut AssetStore,
        _args: KeyPressArgs
    ) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(
        &mut self,
        _asset_store: &mut AssetStore,
        _args: KeyReleaseArgs
    ) {}

    /// Pressed a mouse button.
    fn mouse_press(
        &mut self,
        _asset_store: &mut AssetStore,
        _args: MousePressArgs
    ) {}

    /// Released a mouse button.
    fn mouse_release(
        &mut self,
        _asset_store: &mut AssetStore,
        _args: MouseReleaseArgs
    ) {}

    /// Moved mouse cursor.
    fn mouse_move(
        &mut self,
        _asset_store: &mut AssetStore,
        _args: MouseMoveArgs
    ) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(
        &mut self,
        _asset_store: &mut AssetStore,
        _args: MouseRelativeMoveArgs
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
    &Context::abs(
        args.width as f64, 
        args.height as f64
    ), args),
Update(args) => self.update(asset_store, args),
KeyPress(args) => self.key_press(asset_store, args),
KeyRelease(args) => self.key_release(asset_store, args),
MousePress(args) => self.mouse_press(asset_store, args),
MouseRelease(args) => self.mouse_release(asset_store, args),
MouseMove(args) => self.mouse_move(asset_store, args),
MouseRelativeMove(args) => self.mouse_relative_move(asset_store, args),

                }
            }
        }
    }
}

