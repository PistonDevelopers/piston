
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
    Interactive,
    KeyPress,
    AudioBackEnd,
    SoundSDL2,
    MusicSDL2,
};

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Audio".to_string(),
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

    let music = asset_store.path("background-music.ogg").unwrap();
    let music = MusicSDL2::from_path(&music).unwrap();

    let mut play_sound = false;
    let mut is_playing_background_music = false;

    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Interactive(KeyPress(args)) => {
                    if args.key == keyboard::Space {
                        play_sound = true;
                    }
                },
                Update(args) => {
                    if play_sound {
                        args.audio.play_sound(&sound);
                        play_sound = false;
                    }

                    if !is_playing_background_music {
                        args.audio.play_music(&music);
                        is_playing_background_music = true;
                    }
                },
                _ => {}
            },
        }
    }
}

