
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{
    Gl,
    Texture,
};
use sdl2_game_window::GameWindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let asset_store = AssetStore::from_folder("assets");

    let image = asset_store.path("rust-logo.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    let ref mut gl = Gl::new();
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(args) => {
                    gl.viewport(0, 0, args.width as i32, args.height as i32);

                    let c = Context::abs(args.width as f64, args.height as f64);
                    c.rgb(1.0, 1.0, 1.0).draw(gl);
                    c.image(&image).draw(gl);
                },
                _ => {},       
            },
        }
    }
}


