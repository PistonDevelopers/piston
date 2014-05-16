//! Create window.

use collections::deque::Deque;
use collections::ringbuf::RingBuf;
// External crates.
use glfw;
// Local crate.
use keyboard;
use game_window::{
    event,
    GameWindow,
};
use game_window_settings::GameWindowSettings;

/// Contains stuff for game window.
pub struct GameWindowGLFW {
    /// The window.
    window: glfw::Window,
    /// Receives events from window.
    events: Receiver<(f64, glfw::WindowEvent)>,
    /// GLFW context.
    glfw: glfw::Glfw,
    /// Game window settings;
    settings: GameWindowSettings,

    should_close: bool,
    event_queue: RingBuf<event::Event>,
}

impl GameWindowGLFW {
    fn flush_messages(&mut self) {
        if self.event_queue.len() != 0 {
            return;
        }

        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _)
                if self.settings.exit_on_esc => {
                    self.should_close = true;
                },
                glfw::KeyEvent(key, _, glfw::Press, _) => {
                    self.event_queue.push_back(event::KeyPressEvent(glfw_keycode_to_keycode(key)));
                },
                glfw::KeyEvent(key, _, glfw::Release, _) => {
                    self.event_queue.push_back(event::KeyReleaseEvent(glfw_keycode_to_keycode(key)));
                },
                _ => {},
            }
        }
    }
}

impl GameWindow for GameWindowGLFW {
    fn new(settings: GameWindowSettings) -> GameWindowGLFW {
        use glfw::Context;

        // Create GLFW window.
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (window, events) = glfw.create_window(
            settings.size[0],
            settings.size[1],
            settings.title, glfw::Windowed
        ).expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();

        GameWindowGLFW {
            window: window,
            events: events,
            glfw: glfw,
            settings: settings,

            should_close: false,
            event_queue: RingBuf::<event::Event>::new(),
        }
    }

    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings {
        &self.settings
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn swap_buffers(&self) {
        use glfw::Context;

        self.window.swap_buffers();
    }

    fn poll_event(&mut self) -> event::Event {
        self.flush_messages();

        if self.event_queue.len() != 0 {
            self.event_queue.pop_front().unwrap()
        } else {
            event::NoEvent
        }
    }
}

fn glfw_keycode_to_keycode(keycode: glfw::Key) -> keyboard::Key {
    match keycode {
        glfw::KeySpace => keyboard::Space,
        glfw::KeyEnter => keyboard::Enter,
        glfw::KeyUp => keyboard::Up,
        glfw::KeyDown => keyboard::Down,
        glfw::KeyLeft => keyboard::Left,
        glfw::KeyRight => keyboard::Right,
        _ => keyboard::Unknown,
    }
}

