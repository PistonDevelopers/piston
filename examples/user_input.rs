#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;

use sdl2_game_window::GameWindowSDL2;
use piston::{
    Game,
    GameIteratorSettings,
    GameWindowSettings,
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
};

pub struct App;

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App
    }
}

impl Game for App {
    fn key_press(
        &mut self,
        args: &KeyPressArgs
    ) {
        println!("Pressed keyboard key '{}'", args.key);
    }

    fn key_release(
        &mut self,
        args: &KeyReleaseArgs
    ) {
        println!("Released keyboard key '{}'", args.key);
    }

    fn mouse_press(
        &mut self,
        args: &MousePressArgs
    ) {
        println!("Pressed mouse button '{}'", args.button);
    }

    fn mouse_release(
        &mut self,
        args: &MouseReleaseArgs
    ) {
        println!("Released mouse button '{}'", args.button);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}

