
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
            Err(_) => {
                Err(format!("Could not load '{}'", path.filename_str().unwrap()))
            }
        }
    }
}

impl Sound for SoundSDL2 {
}

/// Wraps SDL2_mixer music data.
pub struct MusicSDL2;

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

impl Drop for AudioSDL2 {
    fn drop(&mut self) {
        mix::quit();
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
}

