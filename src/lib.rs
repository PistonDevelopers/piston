#![crate_name = "piston"]
#![deny(missing_doc)]
#![warn(dead_code)]
#![feature(default_type_params)]
#![feature(globs)]

//! A user friendly graphics engine.

extern crate time;
extern crate sync;

// Crates used to reexport.
extern crate vecmath_lib = "vecmath";
extern crate shader_version_lib = "shader_version";
extern crate image_lib = "image";
extern crate graphics_lib = "graphics";
extern crate input_lib = "input";
extern crate cam_lib = "cam";
extern crate noise_lib = "noise";
extern crate genmesh_lib = "genmesh";

// Reexports.
pub use shader_version_lib as shader_version;
pub use image_lib as image;
pub use graphics_lib as graphics;
pub use vecmath_lib as vecmath;
pub use input_lib as input;
pub use cam_lib as cam;
pub use noise_lib as noise;
pub use genmesh_lib as genmesh;

pub use game_window::{
    GameWindow,
    GameWindowSettings,
    NoGameWindow
};
pub use asset_store::AssetStore;
pub use game_iterator::{
    Render,
    Update,
    Input,
    GameEvent,
    GameIterator,
    GameIteratorSettings,
    RenderArgs,
    UpdateArgs,
};
pub use fps_counter::FPSCounter;

pub mod game_window;
mod game_iterator;
mod asset_store;
mod fps_counter;
