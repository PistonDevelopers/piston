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

// Reexports.
pub use shader_version_lib as shader_version;
pub use image_lib as image;
pub use graphics_lib as graphics;
pub use vecmath_lib as vecmath;
pub use input_lib as input;

pub use camera::{
    Camera, 
    CameraPerspective,
    model_view_projection,
};
pub use fps_controller::{FPSController, FPSControllerSettings};
pub use game_window::GameWindow;
pub use game_window::GameWindowSettings;
pub use asset_store::AssetStore;

pub use game_iterator::Render;
pub use game_iterator::Update;
pub use game_iterator::Input;

pub use game_iterator::GameEvent;
pub use game_iterator::GameIterator;
pub use game_iterator::GameIteratorSettings;
pub use game_iterator::RenderArgs;
pub use game_iterator::UpdateArgs;

pub mod game_window;
pub mod fps_controller;

mod camera;
mod game_iterator;
mod asset_store;

