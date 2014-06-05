
#![feature(globs)]

///! Use SPACE to play the sound.

extern crate graphics;
extern crate piston;

use piston::{
    keyboard,
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    Update,
    KeyPress,
    AudioBackEnd,
    SoundSDL2,
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

    let asset_store = AssetStore::from_folder("assets");

    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);

    let sound = asset_store.path("beeep.ogg").unwrap();
    let sound = SoundSDL2::from_path(&sound).unwrap();

    let mut is_play_sound = false;

    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                KeyPress(args) => {
                    if args.key == keyboard::Space {
                        is_play_sound = true;
                    }
                },
                Update(args) => {
                    if is_play_sound {
                        args.audio.play_sound(&sound);
                        is_play_sound = false;
                    }
                },
                _ => {}
            },
        }
    }
}


