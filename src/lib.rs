#![crate_name = "piston"]
#![deny(missing_doc)]
#![warn(dead_code)]
#![feature(default_type_params)]
#![feature(globs)]

//! A user friendly graphics engine.

extern crate time;
extern crate sync;
extern crate uuid;

// Crates used to reexport.
extern crate "vecmath" as vecmath_lib;
extern crate "shader_version" as shader_version_lib;
extern crate "image" as image_lib;
extern crate "graphics" as graphics_lib;
extern crate "input" as input_lib;
extern crate "event" as event_lib;
extern crate "cam" as cam_lib;
extern crate "noise" as noise_lib;
extern crate "genmesh" as genmesh_lib;

// Reexports.
pub use shader_version_lib as shader_version;
pub use image_lib as image;
pub use graphics_lib as graphics;
pub use vecmath_lib as vecmath;
pub use input_lib as input;
pub use event_lib as event;
pub use cam_lib as cam;
pub use noise_lib as noise;
pub use genmesh_lib as genmesh;

pub use asset_store::AssetStore;
pub use event::{
    Render,
    Update,
    Input,
    Event,
    EventIterator,
    EventSettings,
    NoWindow,
    RenderArgs,
    UpdateArgs,
    Window,
    WindowSettings,
};
pub use fps_counter::FPSCounter;
pub use sprite::Sprite;

mod asset_store;
mod fps_counter;
mod sprite;
