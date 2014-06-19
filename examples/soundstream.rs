//! soundstream.rs
//!
//! A real-time soundstream demo that smoothly copies input
//! from a microphone straight to the output. If <Space> is
//! pressed, the SoundStream thread will begin to calculate
//! and print the samplerate to demonstrate the inter-task
//! event handling.
//!
//! Note: Beware of feedback!

#![feature(globs)]

extern crate piston;

use piston::{
    keyboard,
    AssetStore,
    Game,
    GameEvent,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    KeyPress,
    KeyPressArgs,
    SoundStream,
    soundstreamer
};

//------------------------------

static SAMPLE_RATE: f64 = 44100f64;
static NUM_FRAMES: u32 = 1024u32;
static NUM_CHANNELS: i32 = 2i32;

// Structs
//------------------------------

/// Main application struct.
pub struct App {
    /// Channel for sending information to the audio stream.
    stream_chan: Option<Sender<GameEvent<'static>>> // Channel for sending Events.
}

/// The audio is non-blocking and needs it's own struct.
pub struct AppSoundStream {
    /// Channel for receiving game events from main game stream.
    chan: Option<Receiver<GameEvent<'static>>>, // Channel for receiving Events.
    is_exit: bool, // Trigger for closing the stream.
    is_print: bool, // Toggle for printing the sample_rate.
    buffer: Vec<f32> // Buffer for passing input to output.
}

// Game Method Implementations
//------------------------------

impl Game for App {

    /// Setup / load the app stuff ready for the main loop.
    /// If using a SoundStream, it must be created within this method.
    fn load(&mut self, asset_store: &mut AssetStore) {

        // Create a channel for communicating events with the soundstream.
        // Note: this channel is used for sending InteractiveEvents, but
        // the same technique could be used here to create custom channels
        // that can safely send any kind of unique data.
        let (send, recv) = channel();
        self.stream_chan = Some(send);

        // Create the soundstream on it's own thread for non-blocking, real-time audio.
        // "soundstreamer" will setup and iterate soundstream using portaudio.
        spawn(proc() {
            let mut soundstream = AppSoundStream::new(Some(recv));
            soundstreamer(SAMPLE_RATE, NUM_FRAMES, NUM_CHANNELS, &mut soundstream);
        });

    }

    /// Keypress callback.
    fn key_press(&mut self, args: &KeyPressArgs) {
        println!("Game thread key: {}", args.key);
    }

    /*
    /// Specify the event sending channel. This must be done if we wish
    /// to send interactive events to the SoundStream.
    fn get_event_sender(&self) -> Option<Sender<GameEvent<'static>>> {
        self.stream_chan.clone()
    }
    */
}

impl Drop for App {
    /// Tell the soundstream to exit when App is destroyed.
    fn drop(&mut self) {
        let chan = self.stream_chan.clone();
        match chan {
            Some(sender) => sender.send(KeyPress(KeyPressArgs { key: keyboard::Escape })),
            None => ()
        }
    }
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App {
            stream_chan: None
        }
    }
}

// SoundStream Method Implementations
//------------------------------

impl SoundStream for AppSoundStream {

    /// Load (called prior to main soundstream loop).
    fn load(&mut self) {
        println!("Press <Spacebar> to start/stop printing the real-time sample rate.");
    }

    /// Update (gets called prior to audio_in/audio_out).
    fn update(&mut self, dt: u64) {
        if self.is_print {
            let dtsec: f64 = dt as f64 / 1000000000f64;
            println!("Real-time sample rate: {}", (1f64 / dtsec) * NUM_FRAMES as f64);
        }
    }

    /// AudioInput
    fn audio_in(&mut self, input: &Vec<f32>, num_frames: u32, num_channels: i32) {
        self.buffer = input.clone();
    }

    /// AudioOutput
    fn audio_out(&mut self, output: &mut Vec<f32>, num_frames: u32, num_channels: i32) {
        *output = self.buffer.clone()
    }

    /// KeyPress
    fn key_press(&mut self, args: &KeyPressArgs) {
        println!("Soundstream thread key: {}", args.key);
        if args.key == keyboard::Space {
            let b_print = if self.is_print { false } else { true };
            self.is_print = b_print;
        }
        if args.key == keyboard::Escape {
            self.is_exit = true;
        }
    }

    /*
    /// Retrieve Events for callback (i.e. mouse, keyboard).
    fn check_for_event(&self) -> Option<GameEvent<'static>> {
        match self.chan {
            Some(ref receiver) => match receiver.try_recv() {
                Ok(event) => Some(event),
                Err(_) => None
            },
            None => None
        }
    }
    */

    /// Setup the exit condition (is checked once per buffer).
    fn exit(&self) -> bool { self.is_exit }

}

impl AppSoundStream {
    /// AppSoundStream constructor.
    pub fn new(recv: Option<Receiver<GameEvent<'static>>>) -> AppSoundStream {
        AppSoundStream {
            chan: recv,
            is_exit: false,
            is_print: false,
            buffer: vec![]
        }
    }
}

// Main
//------------------------------

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "soundstream".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [0.1, 0.1, 0.1, 0.1],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");
    let mut app = App::new();
    app.run(&mut window, &mut asset_store);
}


//------------------------------
