//! SoundStream (real-time audio IO).

use GameEvent;
use KeyPress;
use KeyPressArgs;
use KeyRelease;
use KeyReleaseArgs;
use MouseMove;
use MouseMoveArgs;
use MouseRelativeMove;
use MouseRelativeMoveArgs;
use MousePress;
use MousePressArgs;
use MouseRelease;
use MouseReleaseArgs;

use port_audio_back_end::StreamPA;
use time::precise_time_ns;


/// Settings required for SoundStream.
pub struct SoundStreamSettings {
    /// The number of samples per second.
    pub samples_per_second: f64,

    /// How many samples per channel requested at a time in the buffer.
    /// The more frames, the less likely to make glitches,
    /// but this gives slower response.
    pub frames: u32,
    
    /// Number of channels, for example 2 for stereo sound (left + right speaker).
    pub channels: i32
}

impl SoundStreamSettings {
    /// Custom constructor for the SoundStreamSettings.
    ///
    /// ToDo: It would be good to include a method that checked
    /// the feasibility of the requested settings (i.e. that
    /// channels isn't 500, and that samples_per_second and frames
    /// are of a sound card standard).
    pub fn new(samples_per_second: f64, frames: u32, channels: i32)
        -> SoundStreamSettings {
        SoundStreamSettings {
            samples_per_second: samples_per_second,
            frames: frames,
            channels: channels
        }
    }
    /// Default, standard constructor for SoundStreamSettings.
    pub fn cd_quality() -> SoundStreamSettings {
        SoundStreamSettings {
            samples_per_second: 44100f64,
            frames: 512u32,
            channels: 2i32
        }
    }
}


/// Implement this for your real-time audio IO engine.
pub trait SoundStream {

    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {}

    /// Update the physical state of the SoundStream.
    fn update(&mut self, settings: &SoundStreamSettings, dt: u64) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _args: &KeyPressArgs) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _args: &KeyReleaseArgs) {}

    /// Pressed a mouse button.
    fn mouse_press(&mut self, _args: &MousePressArgs) {}

    /// Released a mouse button.
    fn mouse_release(&mut self, _args: &MouseReleaseArgs) {}

    /// Moved mouse cursor.
    fn mouse_move(&mut self, _args: &MouseMoveArgs) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: &MouseRelativeMoveArgs) {}

    /// Offers input via buffer of interleaved f32 samples (amplitude between -1 to 1).
    /// The input buffer's size is num_frames * num_channels.
    /// Get's called at a rate of (sample_rate / num_frames)hz.
    fn audio_in(&mut self, _input: &Vec<f32>, settings: &SoundStreamSettings) {}

    /// Requests output via buffer as interleaved f32 samples (amplitude between -1 to 1).
    /// The output buffer's size is num_frames * num_channels.
    /// Get's called at a rate of (sample_rate / num_frames)hz.
    fn audio_out(&mut self, _output: &mut Vec<f32>, settings: &SoundStreamSettings) {}

    /// Override this using a Receiver<GameEvent> to receive GameEvents from main app.
    fn check_for_event(&self) -> Option<GameEvent> { None }

    /// Override this with your exit condition for the soundstream task.
    fn exit(&self) -> bool { false }

    /// Handles a game event.
    fn event(&mut self, event: &mut GameEvent) {
        match *event {
            KeyPress(ref args) => self.key_press(args),
            KeyRelease(ref args) => self.key_release(args),
            MousePress(ref args) => self.mouse_press(args),
            MouseRelease(ref args) => self.mouse_release(args),
            MouseMove(ref args) => self.mouse_move(args),
            MouseRelativeMove(ref args) => self.mouse_relative_move(args),
            _ => {},
        }
    }

    /// Executes the SoundStream loop.
    fn run(&mut self, settings: SoundStreamSettings) {
        let mut stream_pa = StreamPA::new();
        stream_pa.setup(&settings);
        //stream_pa.run(settings, self);
        self.load();
        stream_pa.start();
        let mut last_time: u64 = precise_time_ns();
        let mut this_time: u64;
        let mut diff_time: u64;
        loop {
            let event = self.check_for_event();
            match event {
                Some(mut e) => self.event(&mut e),
                None => ()
            }
            this_time = precise_time_ns();
            diff_time = this_time - last_time;
            last_time = this_time;
            self.update(&settings, diff_time);
            if self.exit() {
                stream_pa.is_streaming.set(false);
                break;
            }
            else if stream_pa.is_streaming.get() {
                stream_pa.callback(&settings, self);
            }
        }
        stream_pa.stop();
    }

}

