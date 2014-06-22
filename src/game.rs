//! Game loop.

// Local crate.
use GameEvent;
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
    fn render(&mut self, _args: &mut RenderArgs) {}

    /// Update the physical state of the game.
    ///
    /// `dt` is the delta time from last update in seconds.
    fn update(&mut self, _args: &mut UpdateArgs) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self, _asset_store: &mut AssetStore) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _args: &KeyPressArgs) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _args: &KeyReleaseArgs) {}

    /// Pressed a mouse button.
    fn mouse_press(&mut self, _args: &MousePressArgs) {}

    /// Released a mouse button.
    fn mouse_release(&mut self, _args: &MouseReleaseArgs) {}

    /// Moved mouse cursor.
    fn mouse_move(&mut self, _args: &MouseMoveArgs) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: &MouseRelativeMoveArgs) {}

    /// Handles a game event.
    fn event(&mut self, event: &mut GameEvent) {
        match *event {
            Render(ref mut args) => self.render(args),
            Update(ref mut args) => self.update(args),
            KeyPress(ref args) => self.key_press(args),
            KeyRelease(ref args) => self.key_release(args),
            MousePress(ref args) => self.mouse_press(args),
            MouseRelease(ref args) => self.mouse_release(args),
            MouseMove(ref args) => self.mouse_move(args),
            MouseRelativeMove(ref args) => self.mouse_relative_move(args),
        }
    }

    /// Executes a game loop.
    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        let mut game_iter = GameIterator::new(
            game_window,
            &GameIteratorSettings {
                updates_per_second: 120,
                max_frames_per_second: 60
            });

        self.load(asset_store);

        loop {
            match game_iter.next() {
                None => break,
                Some(mut e) => self.event(&mut e)
            }
        }
    }
}

