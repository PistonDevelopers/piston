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
extern crate log;
extern crate collections;
extern crate glfw;
extern crate sdl2;

pub use Game = game::Game;
pub use GameWindow = game_window::GameWindow;
pub use GameWindowSDL2 = game_window_sdl2::GameWindowSDL2;
pub use GameWindowGLFW = game_window_glfw::GameWindowGLFW;
pub use GameWindowSettings = game_window_settings::GameWindowSettings;
pub use Gl = gl::Gl;
pub use GlData = gl::GlData;
pub use AssetStore = asset_store::AssetStore;

pub mod shader_utils;
pub mod game_window;
pub mod keyboard;

mod game;
mod game_window_sdl2;
mod game_window_glfw;
mod game_window_settings;
mod gl;
mod asset_store;

// Temporary copied code from other projects.
mod png;
#[path = "png/inflate.rs"] mod inflate;

