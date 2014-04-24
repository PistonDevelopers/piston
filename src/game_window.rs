//! Creating of window.

use glfw;
use glfw::Context;

/// Contains stuff for game window.
pub struct GameWindow {
    /// The window.
    pub window: glfw::Window,
    /// Receives events from window.
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    /// GLFW context.
    pub glfw: glfw::Glfw,
}

impl GameWindow {
    /// Creates a window.
    #[allow(dead_code)]
    pub fn window(
        title: &str,
        width: u32,
        height: u32
    ) -> GameWindow {
	    // Create GLFW window.
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (window, events) = glfw.create_window(
            width, height, title, glfw::Windowed)
                .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();

        GameWindow {
            window: window,
            events: events,
            glfw: glfw,
        }
    }

    /// Opens up in fullscreen on default monitor.
    /// Sets screen resolution to the physical size of the monitor.
    #[allow(dead_code)]
    pub fn fullscreen(
        title: &str
    ) -> GameWindow { 
	    // Create GLFW window.
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.with_primary_monitor(|m| {
            let m = m.unwrap();
            let (width, height) = m.get_physical_size();
            let (window, events) = glfw.create_window(
                width as u32, height as u32, title, 
                glfw::FullScreen(m)).expect("Failed to create GLFW window.");
            window.set_key_polling(true);
            window.make_current();
        
            GameWindow {
                window: window,
                events: events,
                glfw: glfw,
            }
        })
    }
}

