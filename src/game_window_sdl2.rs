//! A window implemented by SDL2 back-end.

// External crates.
use sdl2;

// Local crate.
use game_window::{
    GameWindow,
};
use event;
use game_window_settings::GameWindowSettings;
use keyboard;

/// A widow implemented by SDL2 back-end.
pub struct GameWindowSDL2 {
    window: sdl2::video::Window,
    context: sdl2::video::GLContext,

    settings: GameWindowSettings,
    should_close: bool,
}

impl GameWindow for GameWindowSDL2 {
    fn new(settings: GameWindowSettings) -> GameWindowSDL2 {
        sdl2::video::gl_set_attribute(sdl2::video::GLContextMajorVersion, 3);
        sdl2::video::gl_set_attribute(sdl2::video::GLContextMinorVersion, 3);
        sdl2::video::gl_set_attribute(sdl2::video::GLContextProfileMask, sdl2::video::ll::SDL_GL_CONTEXT_PROFILE_CORE as int);

        let window = sdl2::video::Window::new(
            settings.title,
            sdl2::video::PosCentered,
            sdl2::video::PosCentered,
            settings.size[0] as int,
            settings.size[1] as int,
            sdl2::video::OpenGL
        ).unwrap();

        let context = window.gl_create_context().unwrap();

        GameWindowSDL2 {
            window: window,
            context: context,

            settings: settings,
            should_close: false,
        }
    }

    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings {
        &self.settings
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }

    fn poll_event(&mut self) -> event::Event {
        match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => { self.should_close = true; },
            sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                if self.settings.exit_on_esc && key == sdl2::keycode::EscapeKey {
                    self.should_close = true;
                } else {
                    return event::KeyPressed(sdl2_keycode_to_keycode(key));
                }
            },
            sdl2::event::KeyUpEvent(_, _, key, _, _) => {
                return event::KeyReleased(sdl2_keycode_to_keycode(key));
            },
            _ => {},
        }
        event::NoEvent
    }
}

fn sdl2_keycode_to_keycode(keycode: sdl2::keycode::KeyCode) -> keyboard::Key {
    match keycode {
        sdl2::keycode::UpKey => keyboard::Up,
        sdl2::keycode::DownKey => keyboard::Down,
        sdl2::keycode::LeftKey => keyboard::Left,
        sdl2::keycode::RightKey => keyboard::Right,
        sdl2::keycode::ReturnKey => keyboard::Enter,
        sdl2::keycode::SpaceKey => keyboard::Space,
        _ => keyboard::Unknown,
    }
}

