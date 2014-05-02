//! Create window.

// External crates.
use glfw;

// Local crate.
use game_window_settings::GameWindowSettings;

/// Contains stuff for game window.
pub struct GameWindow {
    /// The window.
    pub window: glfw::Window,
    /// Receives events from window.
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    /// GLFW context.
    pub glfw: glfw::Glfw,
    /// Game window settings;
    pub settings: GameWindowSettings,
}

impl GameWindow {
    /// Creates a window.
    #[allow(dead_code)]
    pub fn window(
        title: &str,
        width: u32,
        height: u32,
        settings: GameWindowSettings
    ) -> GameWindow {
        use glfw::Context;	    

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
            settings: settings,
        }
    }

    /// Opens up in fullscreen on default monitor.
    /// Sets screen resolution to the physical size of the monitor.
    #[allow(dead_code)]
    pub fn fullscreen(
        title: &str,
        settings: GameWindowSettings
    ) -> GameWindow { 
	    // Create GLFW window.
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.with_primary_monitor(|m| {
            use glfw::Context;            

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
                settings: settings,
            }
        })
    }
}

