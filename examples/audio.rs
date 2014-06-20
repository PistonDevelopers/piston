
#![feature(globs)]

extern crate piston;

use piston::{
    keyboard,
    AssetStore,
    AudioBackEnd,
    AudioSDL2,
    Game,
    GameWindowSDL2,
    GameWindowSettings,
    SoundSDL2,
    MusicSDL2,
    UpdateArgs,
    KeyPressArgs,
};

pub struct App {
    play_sound: bool,
    is_playing_background_music: bool,
    audio: AudioSDL2,
    sound: Option<SoundSDL2>,
    music: Option<MusicSDL2>,
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App {
            play_sound: false,
            is_playing_background_music: false,
            audio: AudioSDL2::new(),
            sound: None,
            music: None,
        }
    }
}

impl Game for App {
    fn load(&mut self, asset_store: &mut AssetStore) {
        let sound = asset_store.path("beeep.ogg").unwrap();
        self.sound = Some(SoundSDL2::from_path(&sound).unwrap());

        let music = asset_store.path("background-music.ogg").unwrap();
        self.music = Some(MusicSDL2::from_path(&music).unwrap());
    }

    fn update(&mut self, _args: &mut UpdateArgs) {
        if !self.is_playing_background_music {
            self.audio.play_music(self.music.get_ref());
            self.is_playing_background_music = true;
        }

        if self.play_sound {
            self.audio.play_sound(self.sound.get_ref());
            self.play_sound = false;
        }
    }

    fn key_press(&mut self, args: &KeyPressArgs) {
        if args.key == keyboard::Space {
            self.play_sound = true;
        }
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
            title: "Audio".to_string(),
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

