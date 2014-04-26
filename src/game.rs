//! Game loop.

use glfw;
use glfw::Context;
use gl = opengles::gl2;
use graphics;
use Gl = gl::Gl;

use game_window::GameWindow;

/// Basic settings for window behavior.
pub struct Settings {
    /// If true, exit when pressing Esc.
    pub exit_on_esc: bool,
    /// The color to use as background.
    pub background_color: [f32, ..4],
}

impl Settings {
    /// Gets default settings.
    pub fn default() -> Settings {
        Settings {
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Creates a new Settings.
    pub fn new(exit_on_esc: bool, background_color: [f32, ..4]) -> Settings {
        Settings {
            exit_on_esc: exit_on_esc,
            background_color: background_color,
        }
    }
}

/// Implement default behavior for a game.
pub trait Game {
    /// Read settings.
    fn get_settings<'a>(&'a self) -> &'a Settings;
    
    /// Render graphics.
    fn render(&self, context: &graphics::Context, gl: &mut Gl); 
    
    /// Update the physical state of the game.
    fn update(&mut self);
    
    /// Perform tasks for loading before showing anything.
    fn load(&mut self);

    /// User pressed a key.
    fn key_press(&mut self, _key: glfw::Key) {}

    /// User released a key.
    fn key_release(&mut self, _key: glfw::Key) {}

    /// Sets up viewport.
    #[inline(always)]
    fn viewport(&self, game_window: &GameWindow) {
        let (w, h) = game_window.window.get_size();
        gl::viewport(0, 0, w as gl::GLint, h as gl::GLint); 
    }

    /// Whether the window should be closed.
    fn should_close(&self, game_window: &GameWindow) -> bool {
        game_window.window.should_close()
    }

    /// Swaps the front buffer with the back buffer.
    /// This shows the next frame.
    fn swap_buffers(&self, game_window: &GameWindow) {
        game_window.window.swap_buffers()
    }

    /// Handles events with default settings..
    fn handle_events(&mut self, game_window: &GameWindow) {
        let exit_on_esc = self.get_settings().exit_on_esc;
        game_window.glfw.poll_events();
        for (_, event) in 
        glfw::flush_messages(&game_window.events) {
            match event {
                // Close with Esc.
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _)
                if exit_on_esc  => {
                    game_window.window.set_should_close(true)
                },
                glfw::KeyEvent(key, _, glfw::Press, _) => {
                    self.key_press(key)
                },
                glfw::KeyEvent(key, _, glfw::Release, _) => {
                    self.key_release(key)
                },
                _ => {},
            }
        }
    }

    /// Executes a game loop.
    fn run(&mut self, game_window: &GameWindow) {
        use graphics::{Clear, AddColor};
        use gl::Gl;

        self.load();
        let mut gl = Gl::new();
        let context = graphics::Context::new();
        let bg = self.get_settings().background_color;
        let bg = context.rgba(bg[0], bg[1], bg[2], bg[3]);
        while !self.should_close(game_window) {
            self.viewport(game_window);
            bg.clear(&mut gl);
            self.render(&context, &mut gl);
            self.swap_buffers(game_window);
            self.update();
            self.handle_events(game_window);
        }
    }
}

