
/**
 *
 * PortAudio backend for Piston!
 *
 * */

#[cfg(portaudio)]
use portaudio::*;
use std::cell::Cell;
use sound_stream::SoundStream;
use time::precise_time_ns;

//------------------------------

/// PortAudio Stream Parameters (required to setup stream).
pub struct StreamParamsPA {
    input: types::PaStreamParameters,
    output: types::PaStreamParameters,
}

/// PortAudio Stream (for reading from and writing to real-time audio stream).
pub struct StreamPA {
    stream: pa::PaStream<f32>,
    is_streaming: Cell<bool>
}

//------------------------------

impl StreamParamsPA {

    /// Creates the port audio stream parameters.
    pub fn new(num_channels: i32) -> StreamParamsPA {

        //println!("Portaudio version : {}", pa::get_version());
        //println!("Portaudio version text : {}", pa::get_version_text());
        //println!("Portaudio error text : {}", pa::get_error_text(types::PaNotInitialized));

        println!("Portaudio init error : {}", pa::get_error_text(pa::initialize()));

        /*let host_count = pa::get_host_api_count();
        println!("Portaudio host count : {}", host_count as int);

        let default_host = pa::get_default_host_api();
        println!("Portaudio default host : {}", default_host as int);

        let host_info = pa::get_host_api_info(default_host);
        println!("Portaudio host name : {}", host_info.unwrap().name);

        println!("Portaudio type id : {}",
                 pa::host_api_type_id_to_host_api_index(types::PaCoreAudio) as int);
        */

        println!("Creating StreamParamsPA");
        let def_input = pa::get_default_input_device();          
        let def_output = pa::get_default_output_device();

        println!("Creating input");
        let stream_params_in = types::PaStreamParameters {
            device: def_input,
            channel_count: 2,
            sample_format: types::PaFloat32,
            suggested_latency: pa::get_device_info(def_input).unwrap().default_low_input_latency
        };
        println!("Creating output");
        let stream_params_out = types::PaStreamParameters {
            device: def_output,
            channel_count: num_channels,
            sample_format: types::PaFloat32,
            suggested_latency: pa::get_device_info(def_output).unwrap().default_low_output_latency
        };
        StreamParamsPA {
            input: stream_params_in,
            output: stream_params_out
        }
    }

    /// Prints information about the port audio host and stream.
    pub fn print_info(&self) {
        println!("Portaudio version: {}", pa::get_version());
        println!("Portaudio version text: {}", pa::get_version_text());
        println!("Portaudio error text: {}", pa::get_error_text(types::PaNotInitialized));
        println!("Portaudio init error : {}", pa::get_error_text(pa::initialize()));
        let host_count = pa::get_host_api_count();
        println!("Portaudio host count : {}", host_count as int);
        let default_host = pa::get_default_host_api();
        println!("Portaudio default host : {}", default_host as int);
        let host_info = pa::get_host_api_info(default_host);
        println!("Portaudio host name : {}", host_info.unwrap().name);
        println!("Portaudio type id : {}",
                 pa::host_api_type_id_to_host_api_index(types::PaCoreAudio) as int);
        let info_input = pa::get_device_info(self.input.device).unwrap();
        println!("Default input device info :");                             
        println!("version : {}", info_input.struct_version);                 
        println!("Input name : {}", info_input.name);                              
        println!("max input channels : {}", info_input.max_input_channels);  
        println!("max output channels : {}", info_input.max_output_channels);
        println!("default sample rate : {}", info_input.default_sample_rate);
    }

}

//------------------------------

impl StreamPA {

    /// Constructor for the portaudio stream.
    pub fn new() -> StreamPA {
        StreamPA {
            stream : pa::PaStream::new(types::PaFloat32),
            is_streaming : Cell::new(true)
        }
    }

    /// Setup the portaudio stream.
    pub fn setup(&mut self, sample_rate: f64, num_frames: u32, num_channels: i32) {
        let params = StreamParamsPA::new(num_channels);
        self.stream.open(Some(&params.input),
                         Some(&params.output),
                         sample_rate,
                         num_frames,
                         types::PaClipOff);
    }

    /// Runs the portaudio stream stuff.
    fn run<T: SoundStream>(&mut self, num_frames: u32, num_channels: i32, sound: &mut T) {
        sound.load();
        self.start();
        let mut last_time: u64 = precise_time_ns();
        let mut this_time: u64;
        let mut diff_time: u64;
        loop {
            let event = sound.check_for_event();
            match event {
                Some(mut e) => sound.event(&mut e),
                None => ()
            }
            this_time = precise_time_ns();
            diff_time = this_time - last_time;
            last_time = this_time;
            sound.update(diff_time);
            if sound.exit() {
                self.is_streaming.set(false);
                break;
            }
            else if self.is_streaming.get() {
                self.callback(num_frames, num_channels, sound);
            }
        }
        self.stop();
    }

    /// Performs the audio read/write.
    pub fn callback<T: SoundStream>(&mut self, num_frames: u32, num_channels: i32, sound: &mut T) {
        let mut ready = 0;
        while ready == 0 {
            ready = self.stream.get_stream_write_available();
        }
        let mut read: Vec<f32> = Vec::from_elem((num_frames * num_channels as u32) as uint, 0f32);
        self.read(&mut read, num_frames, num_channels, sound);
        let mut write: Vec<f32> = Vec::from_elem((num_frames * num_channels as u32) as uint, 0f32);
        self.write(&mut write, num_frames, num_channels, sound);
    }

    /// Read audio in from stream.
    pub fn read<T: SoundStream>(&self, buffer: &mut Vec<f32>, num_frames: u32,
                                num_channels: i32, sound: &mut T) {
        *buffer = match self.stream.read(num_frames as u32) {
            Ok(res) => {
                sound.audio_in(&res, num_frames, num_channels);
                res
            },
            Err(err) => {
                fail!(format!("Portaudio error read : {}", pa::get_error_text(err)));
            }
        };
    }

    /// Write audio to stream
    pub fn write<T: SoundStream>(&mut self, buffer: &mut Vec<f32>, num_frames: u32,
                                 num_channels: i32, sound: &mut T) {
        sound.audio_out(buffer, num_frames, num_channels);
        let write: Vec<f32> = buffer.clone();
        self.stream.write(write, num_frames);
    }

    /// Start the audio stream.
    pub fn start(&mut self) {
        let err = self.stream.start();
        println!("Portaudio Start Stream : {}", pa::get_error_text(err));
    }

    /// Stop the audio stream.
    pub fn stop(&mut self) {
        let mut err = types::PaNotInitialized;
        err = self.stream.close();
        println!("Portaudio Closing Stream : {}", pa::get_error_text(err));
        println!("Portaudio Termination Message : {}", pa::get_error_text(pa::terminate()));
    }

}

/// Ensure that the stream closes properly upon object destruction.
impl Drop for StreamPA {
    fn drop(&mut self) {
        if self.is_streaming.get() {
            self.stop();
        }
    }
}

//------------------------------

/// Run this in a separate task to initiate the realtime input/output
/// audio stream. The SoundStream type will be iterated using the
/// portaudio stream.
pub fn soundstreamer<T: SoundStream>(sample_rate: f64, num_frames: u32,
                                     num_channels: i32, sound: &mut T) {
    let mut stream_pa = StreamPA::new();
    stream_pa.setup(sample_rate, num_frames, num_channels);
    stream_pa.run(num_frames, num_channels, sound);
}

//------------------------------
