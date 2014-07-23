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
pub trait Game<W: GameWindow> {
    /// Render graphics.
    fn render(&mut self, _window: &mut W, _args: &RenderArgs) {}

    /// Update the physical state of the game.
    fn update(&mut self, _window: &mut W, _args: &UpdateArgs) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _window: &mut W, _args: &KeyPressArgs) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _window: &mut W, _args: &KeyReleaseArgs) {}

    /// Pressed a mouse button.
    fn mouse_press(&mut self, _window: &mut W, _args: &MousePressArgs) {}

    /// Released a mouse button.
    fn mouse_release(&mut self, _window: &mut W, _args: &MouseReleaseArgs) {}

    /// Moved mouse cursor.
    fn mouse_move(&mut self, _window: &mut W, _args: &MouseMoveArgs) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _window: &mut W, _args: &MouseRelativeMoveArgs) {}

    /// Scrolled mouse.
    fn mouse_scroll(&mut self, _window: &mut W, _args: &MouseScrollArgs) {}

    /// Whether the cursor should be captured.
    ///
    /// When the cursor is captured, it is hidden from view
    /// and the cursor position does not change.
    /// Only relative mouse movements are registered.
    #[inline(always)]
    fn should_capture_cursor(&mut self) -> bool { false }

    /// Handles a game event.
    fn event(&mut self, game_window: &mut W, event: &mut GameEvent) {
        match *event {
            Render(ref mut args) => self.render(game_window, args),
            Update(ref mut args) => self.update(game_window, args),
            KeyPress(ref args) => self.key_press(game_window, args),
            KeyRelease(ref args) => self.key_release(game_window, args),
            MousePress(ref args) => self.mouse_press(game_window, args),
            MouseRelease(ref args) => self.mouse_release(game_window, args),
            MouseMove(ref args) => self.mouse_move(game_window, args),
            MouseRelativeMove(ref args) => self.mouse_relative_move(game_window, args),
            MouseScroll(ref args) => self.mouse_scroll(game_window, args),
        }
    }

    /// Executes a game loop.
    fn run(
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
                Some(mut e) => self.event(game_iter.game_window, &mut e)
            }
            if  self.should_capture_cursor() != should_capture_cursor {
                should_capture_cursor = !should_capture_cursor;
                game_iter.game_window.capture_cursor(should_capture_cursor);
            }
        }
        game_iter.game_window.capture_cursor(false);
    }
}

