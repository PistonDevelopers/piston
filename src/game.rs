//! Game loop.

// External crates.
use graphics::Context;

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
    fn update(&mut self, _args: UpdateArgs) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self, _asset_store: &mut AssetStore) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _args: KeyPressArgs) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _args: KeyReleaseArgs) {}

    /// Pressed a mouse button.
    fn mouse_press(&mut self, _args: MousePressArgs) {}

    /// Released a mouse button.
    fn mouse_release(&mut self, _args: MouseReleaseArgs) {}

    /// Moved mouse cursor.
    fn mouse_move(&mut self, _args: MouseMoveArgs) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: MouseRelativeMoveArgs) {}

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
                        ), 
                        args
                    ),
                    Update(args) => self.update(args),
                    KeyPress(args) => self.key_press(args),
                    KeyRelease(args) => self.key_release(args),
                    MousePress(args) => self.mouse_press(args),
                    MouseRelease(args) => self.mouse_release(args),
                    MouseMove(args) => self.mouse_move(args),
                    MouseRelativeMove(args) => self.mouse_relative_move(args),

                }
            }
        }
    }
}

