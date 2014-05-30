
#![feature(globs)]

extern crate graphics;
extern crate piston;

use graphics::*;
use piston::{
    AssetStore,
    GameIterator,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    Gl,
    Render,
};

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");

    let image = asset_store.load_image("rust-logo.png").unwrap();
    let mut game_iter = GameIterator::new(&mut window);
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(args) => {
                    let c = args.context;
                    let mut gl = Gl::new(args.gl_data, &mut asset_store);
                    c.image(image).draw(&mut gl);
                },
                _ => {},       
            },
        }
    }
}


