//! PortAudio backend for Piston!


use portaudio::{ types, pa };
use std::cell::Cell;
use sound_stream::SoundStream;
use sound_stream::SoundStreamSettings;


/// PortAudio Stream Parameters (required to setup stream).
pub struct StreamParamsPA {
    input: types::PaStreamParameters,
    output: types::PaStreamParameters,
}

/// PortAudio Stream (for reading from and writing to real-time audio stream).
pub struct StreamPA {
    stream: pa::PaStream<f32>,
    pub is_streaming: Cell<bool>
}

/// The parameters required to set up the PortAudio stream.
impl StreamParamsPA {

    /// Creates the port audio stream parameters.
    pub fn new(channels: i32) -> StreamParamsPA {

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
            channel_count: channels,
            sample_format: types::PaFloat32,
            suggested_latency: pa::get_device_info(def_input).unwrap().default_low_input_latency
        };
        println!("Creating output");
        let stream_params_out = types::PaStreamParameters {
            device: def_output,
            channel_count: channels,
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


impl StreamPA {

    /// Constructor for the portaudio stream.
    pub fn new() -> StreamPA {
        StreamPA {
            stream : pa::PaStream::new(types::PaFloat32),
            is_streaming : Cell::new(true)
        }
    }

    /// Setup the portaudio stream.
    pub fn setup(&mut self, settings: &SoundStreamSettings) {
        let params = StreamParamsPA::new(settings.channels);
        self.stream.open(Some(&params.input),
                         Some(&params.output),
                         settings.samples_per_second,
                         settings.frames,
                         types::PaClipOff);
    }

    /// Performs the audio read/write.
    pub fn callback<T: SoundStream>(&mut self, settings: &SoundStreamSettings, stream: &mut T) {
        let mut ready = 0;
        while ready == 0 {
            ready = self.stream.get_stream_write_available();
        }
        let empty_buffer = Vec::from_elem((settings.frames * settings.channels as u32) as uint, 0f32);
        let mut read: Vec<f32> = empty_buffer.clone();
        self.read(&mut read, settings, stream);
        let mut write: Vec<f32> = empty_buffer.clone();
        self.write(&mut write, settings, stream);
    }

    /// Read audio in from stream.
    pub fn read<T: SoundStream>(&self, buffer: &mut Vec<f32>,
                                settings: &SoundStreamSettings, stream: &mut T) {
        *buffer = match self.stream.read(settings.frames as u32) {
            Ok(in_buffer) => {
                stream.audio_in(&in_buffer, settings);
                in_buffer
            },
            Err(err) => {
                fail!(format!("Portaudio error read : {}", pa::get_error_text(err)));
            }
        };
    }

    /// Write audio to stream
    pub fn write<T: SoundStream>(&mut self, buffer: &mut Vec<f32>,
                                 settings: &SoundStreamSettings, stream: &mut T) {
        stream.audio_out(buffer, settings);
        let write: Vec<f32> = buffer.clone();
        self.stream.write(write, settings.frames);
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

