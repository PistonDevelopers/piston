//! SoundStream (real-time audio IO).

use InteractiveEvent;
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

/// Implement this for your real-time audio IO engine.
pub trait SoundStream {

    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {}

    /// Update the physical state of the SoundStream.
    fn update(&mut self, dt: u64) {}

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
    fn audio_in(&mut self, input: &Vec<f32>,  num_frames: u32, num_channels: i32) {}

    /// Requests output via buffer as interleaved f32 samples (amplitude between -1 to 1).
    /// The output buffer's size is num_frames * num_channels.
    /// Get's called at a rate of (sample_rate / num_frames)hz.
    fn audio_out(&mut self, output: &mut Vec<f32>,  num_frames: u32, num_channels: i32) {}

    /// Override this using a Receiver<GameEvent> to receive GameEvents from main app.
    fn check_for_event(&self) -> Option<InteractiveEvent> { None }

    /// Override this with your exit condition for the soundstream task.
    fn exit(&self) -> bool { false }

    /// Handles a game event.
    fn event(&mut self, event: &mut InteractiveEvent) {
        match *event {
            KeyPress(ref args) => self.key_press(args),
            KeyRelease(ref args) => self.key_release(args),
            MousePress(ref args) => self.mouse_press(args),
            MouseRelease(ref args) => self.mouse_release(args),
            MouseMove(ref args) => self.mouse_move(args),
            MouseRelativeMove(ref args) => self.mouse_relative_move(args),
        }
    }

}

