#![crate_id = "piston"]
#![deny(missing_doc)]
#![feature(globs)]

//! A user friendly graphics engine.

extern crate time;
extern crate graphics;
extern crate opengles;
extern crate glfw;

pub use Game = game::Game;
pub use GameWindow = game_window::GameWindow;
pub use GameWindowSettings = game_window_settings::GameWindowSettings;
pub use Gl = gl::Gl;

pub mod shader_utils;

mod game;
mod game_window;
mod game_window_settings;
mod gl;

