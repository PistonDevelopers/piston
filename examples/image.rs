#![feature(globs)]

extern crate graphics;
extern crate piston;

use graphics::*;
use piston::{
    AssetStore,
    Game,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings, 
    Gl
};

pub struct App {
    image: Option<Image>,
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App {
            image: None,
        }
    }
}

impl Game for App {
    fn load(&mut self, asset_store: &mut AssetStore) {
        self.image = Some(asset_store.load_image("rust-logo.png").unwrap());
    }

    fn render(&self, _ext_dt: f64, c: &Context, gl: &mut Gl) {
        c.image(self.image.unwrap()).draw(gl);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Image".to_owned(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");
    
    let mut app = App::new();
    app.run(&mut window, &mut asset_store);
}


