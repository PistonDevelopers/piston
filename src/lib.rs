#![crate_id = "piston"]
#![deny(missing_doc)]
#![deny(dead_code)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]

//! A user friendly graphics engine.

extern crate time;
extern crate graphics;
extern crate opengles;
extern crate glfw;
extern crate log;
extern crate collections;

pub use Game = game::Game;
pub use GameWindow = game_window::GameWindow;
pub use GameWindowSettings = game_window_settings::GameWindowSettings;
pub use Gl = gl::Gl;
pub use GlData = gl::GlData;
pub use AssetStore = asset_store::AssetStore;

pub mod shader_utils;

mod game;
mod game_window;
mod game_window_settings;
mod gl;
mod asset_store;

// Temporary copied code from other projects.
mod png;
#[path = "png/inflate.rs"] mod inflate;

