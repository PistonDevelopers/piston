//! Game loop.

// External crates.
use time;
use glfw;
use gl = opengles::gl2;
use graphics::*;

// Local crate.
use Gl = gl::Gl;
use GameWindow = game_window::GameWindow;

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
    fn update(&mut self, _dt: f64) {}
    
    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _key: glfw::Key) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _key: glfw::Key) {}

    /// Sets up viewport.
    ///
    /// A viewport is the region of the window where graphics is rendered.  
    #[inline(always)]
    fn viewport(&self, game_window: &GameWindow) {
        let (w, h) = game_window.window.get_size();
        gl::viewport(0, 0, w as gl::GLint, h as gl::GLint); 
    }

    /// Whether the window should be closed.
    ///
    /// When this is `true` the application shuts down.  
    /// This can be overridden to emulate a user closing the window.  
    /// One can also override this method to prevent window from closing.  
    fn should_close(&self, game_window: &GameWindow) -> bool {
        game_window.window.should_close()
    }

    /// Swaps the front buffer with the back buffer.  
    /// 
    /// When called, This shows the next frame.  
    /// The graphics is rendered to the back buffer.  
    /// The front buffer is displayed on the screen.  
    fn swap_buffers(&self, game_window: &GameWindow) {
        use glfw::Context;

        game_window.window.swap_buffers()
    }

    /// Handles events using current game window settings.
    ///
    /// This can be overriden to do custom event handling.  
    fn handle_events(&mut self, game_window: &GameWindow) {
        let exit_on_esc = game_window.settings.exit_on_esc;
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
    /// 
    /// The loop continues until `should_close` returns true.  
    fn run(&mut self, game_window: &GameWindow) {
        use graphics::{Clear, AddColor};
        use gl::Gl;

        self.load();
        let mut gl = Gl::new();
        let context = Context::new();
        let bg = game_window.settings.background_color;
        let bg = context.rgba(bg[0], bg[1], bg[2], bg[3]);
        let updates_per_second: u64 = 100;
        let dt: f64 = 1.0 / updates_per_second as f64;
        let update_time_in_ns: u64 = 1_000_000_000 / updates_per_second;
        let mut last_update = time::precise_time_ns();
        while !self.should_close(game_window) {
            self.viewport(game_window);
            bg.clear(&mut gl);
            self.render(&context, &mut gl);
            self.swap_buffers(game_window);
            // Perform updates by fixed time step until it catches up.
            loop {
                self.update(dt);
                last_update += update_time_in_ns;
                let now = time::precise_time_ns();
                if now <= last_update { break; }
            }
            self.handle_events(game_window);
        }
    }
}

