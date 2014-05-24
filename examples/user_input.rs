
#![feature(globs)]

extern crate graphics;
extern crate piston;

use piston::{
    AssetStore,
    Game,
    GameWindow,
    GameWindowGLFW,
    GameWindowSDL2,
    GameWindowSettings,
    keyboard,
    mouse,
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
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        println!("Pressed keyboard key '{}'", key);
    }

    fn key_release(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        println!("Released keyboard key '{}'", key);
    }

    fn mouse_press(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        println!("Pressed mouse button '{}'", button);
    }

    fn mouse_release(
        &mut self, 
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        println!("Released mouse button '{}'", button);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowGLFW = GameWindow::new(
        GameWindowSettings {
            title: "Keycode".to_owned(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::empty();

    let mut app = App::new();
    app.run(&mut window, &mut asset_store);
}

