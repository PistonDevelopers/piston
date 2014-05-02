#![crate_id = "piston"]
#![deny(missing_doc)]

//! A user friendly graphics engine.

extern crate time;
extern crate graphics;
extern crate opengles;
extern crate glfw;

pub use Game = game::Game;
pub use GameWindow = game_window::GameWindow;
pub use GameSettings = game_settings::GameSettings;
pub use Gl = gl::Gl;

pub mod shader_utils;

mod game;
mod game_settings;
mod game_window;
mod gl;

