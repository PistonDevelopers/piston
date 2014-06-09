
use mix = sdl2_mixer;
use {
    AudioBackEnd,
    Music,
    Sound,
};

/// Wraps SDL2_mixer chunk data.
pub struct SoundSDL2 {
    chunk: mix::Chunk,
    loops: int,
}

impl SoundSDL2 {
    /// Loads sound by relative file name to the asset root.
    pub fn from_path(path: &Path) -> Result<SoundSDL2, String> {
        match mix::Chunk::from_file(path) {
            Ok(chunk) => {
                Ok(SoundSDL2 {
                    chunk: chunk,
                    loops: 1,
                })
            },
            Err(msg) => {
                Err(format!("Could not load '{}': {}", path.filename_str().unwrap(), msg))
            },
        }
    }
}

impl Sound for SoundSDL2 {
}

/// Wraps SDL2_mixer music data.
pub struct MusicSDL2 {
    music: mix::Music,
    loops: int,
}

impl MusicSDL2 {
    /// Loads music by relative file name to the asset root.
    pub fn from_path(path: &Path) -> Result<MusicSDL2, String> {
        match mix::Music::from_file(path) {
            Ok(music) => {
                Ok(MusicSDL2 {
                    music: music,
                    loops: 1,
                })
            },
            Err(msg) => {
                Err(format!("Could not load '{}': {}", path.filename_str().unwrap(), msg))
            },
        }
    }
}

impl Music for MusicSDL2 {
}

/// An audio back end using SDL2_mixer
pub struct AudioSDL2;

impl AudioSDL2 {
    /// Create a new instance
    pub fn new() -> AudioSDL2 {
        mix::init(mix::InitMp3 | mix::InitFlac
                  | mix::InitMod | mix::InitFluidSynth
                  | mix::InitModPlug | mix::InitOgg);
        mix::open_audio(mix::DEFAULT_FREQUENCY,
                        mix::DEFAULT_FORMAT,
                        mix::DEFAULT_CHANNELS,
                        1024).unwrap();
        mix::allocate_channels(mix::DEFAULT_CHANNELS);
        AudioSDL2
    }
}

impl AudioBackEnd<MusicSDL2, SoundSDL2> for AudioSDL2 {
    fn play_sound(&self, sound: &SoundSDL2) {
        match mix::Channel::all().play(&sound.chunk, sound.loops) {
            Err(msg) => {
                println!("Warning: {}", msg);
            },
            _ => {}
        }
    }

    fn play_music(&self, music: &MusicSDL2) {
        match music.music.play(music.loops) {
            Err(msg) => {
                println!("Warning: {}", msg);
            },
            _ => {}
        }
    }
}

