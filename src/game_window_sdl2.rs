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
use mouse;
use gl;

/// A widow implemented by SDL2 back-end.
pub struct GameWindowSDL2 {
    window: sdl2::video::Window,
    context: sdl2::video::GLContext,
    settings: GameWindowSettings,
    should_close: bool,
    last_pressed_key: Option<sdl2::keycode::KeyCode>,
}

impl GameWindow for GameWindowSDL2 {
    fn new(settings: GameWindowSettings) -> GameWindowSDL2 {
        sdl2::video::gl_set_attribute(sdl2::video::GLContextMajorVersion, 3);
        sdl2::video::gl_set_attribute(sdl2::video::GLContextMinorVersion, 3);
        sdl2::video::gl_set_attribute(sdl2::video::GLContextProfileMask, sdl2::video::ll::SDL_GL_CONTEXT_PROFILE_CORE as int);

        let window = sdl2::video::Window::new(
            settings.title.as_slice(),
            sdl2::video::PosCentered,
            sdl2::video::PosCentered,
            settings.size[0] as int,
            settings.size[1] as int,
            sdl2::video::OpenGL
        ).unwrap();

        let context = window.gl_create_context().unwrap();

        // Load the OpenGL function pointers
        gl::load_with(|s| sdl2::video::gl_get_proc_address(s));

        GameWindowSDL2 {
            window: window,
            context: context,
            settings: settings,
            should_close: false,
            last_pressed_key: None,
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
                // SDL2 repeats the key down event.
                // If the event is the same as last one, ignore it.
                match self.last_pressed_key {
                    Some(x) if x == key => return event::NoEvent,
                    _ => {},
                };
                self.last_pressed_key = Some(key);

                if self.settings.exit_on_esc 
                && key == sdl2::keycode::EscapeKey {
                    self.should_close = true;
                } else {
                    return event::KeyPressed(sdl2_map_key(key));
                }
            },
            sdl2::event::KeyUpEvent(_, _, key, _, _) => {
                // Reset the last pressed key.
                self.last_pressed_key = match self.last_pressed_key {
                    Some(x) if x == key => None,
                    x => x,
                };

                return event::KeyReleased(sdl2_map_key(key));
            },
            sdl2::event::MouseButtonDownEvent(_, _, _, button, _, _) => {
                return event::MouseButtonPressed(sdl2_map_mouse(button));
            },
            sdl2::event::MouseButtonUpEvent(_, _, _, button, _, _) => {
                return event::MouseButtonReleased(sdl2_map_mouse(button));
            },
            sdl2::event::MouseMotionEvent(_, _, _, _, x, y, dx, dy) => {
                return event::MouseMoved(
                    x as f64,
                    y as f64,
                    Some((dx as f64, dy as f64))
                );
            },
            _ => {},
        }
        event::NoEvent
    }
}

fn sdl2_map_key(keycode: sdl2::keycode::KeyCode) -> keyboard::Key {
    use std::num::FromPrimitive;
    FromPrimitive::from_u64(keycode.code() as u64).unwrap()
}

fn sdl2_map_mouse(button: sdl2::mouse::Mouse) -> mouse::Button {
    match button {
        sdl2::mouse::LeftMouse => mouse::Left,
        sdl2::mouse::RightMouse => mouse::Right,
        sdl2::mouse::MiddleMouse => mouse::Middle,
        sdl2::mouse::X1Mouse => mouse::X1,
        sdl2::mouse::X2Mouse => mouse::X2,
        sdl2::mouse::UnknownMouse(_) => mouse::Unknown,
    }
}

