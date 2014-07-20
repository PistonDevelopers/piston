//! Game loop.

// Local crate.
use GameEvent;
use GameWindow = game_window::GameWindow;
use GameIteratorSettings;
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
use MouseScroll;
use MouseScrollArgs;
use Render;
use RenderArgs;
use Update;
use UpdateArgs;

/// Implemented by game applications.
pub trait Game {
    /// Render graphics.
    fn render(&mut self, _args: &RenderArgs) {}

    /// Update the physical state of the game.
    fn update(&mut self, _args: &UpdateArgs) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {}

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

    /// Scrolled mouse.
    fn mouse_scroll(&mut self, _args: &MouseScrollArgs) {}

    /// Whether the cursor should be captured.
    ///
    /// When the cursor is captured, it is hidden from view
    /// and the cursor position does not change.
    /// Only relative mouse movements are registered.
    #[inline(always)]
    fn should_capture_cursor(&mut self) -> bool { false }

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
            MouseScroll(ref args) => self.mouse_scroll(args),
        }
    }

    /// Executes a game loop.
    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        game_iter_settings: &GameIteratorSettings
    ) {
        let mut game_iter = GameIterator::new(
            game_window,
            game_iter_settings
        );

        self.load();

        let mut should_capture_cursor = self.should_capture_cursor();
        game_iter.game_window.capture_cursor(should_capture_cursor);
        loop {
            match game_iter.next() {
                None => break,
                Some(mut e) => self.event(&mut e)
            }
            if  self.should_capture_cursor() != should_capture_cursor {
                should_capture_cursor = !should_capture_cursor;
                game_iter.game_window.capture_cursor(should_capture_cursor);
            }
        }
        game_iter.game_window.capture_cursor(false);
    }
}

