#![crate_id = "piston"]
#![deny(missing_doc)]
#![deny(dead_code)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]

//! A user friendly graphics engine.

extern crate time;
extern crate graphics;
extern crate log;
extern crate collections;
extern crate gl;
extern crate glfw;
extern crate sdl2;
extern crate png;
extern crate libc;

pub use Game = game::Game;
pub use GameWindow = game_window::GameWindow;
pub use GameWindowSDL2 = game_window_sdl2::GameWindowSDL2;
pub use GameWindowGLFW = game_window_glfw::GameWindowGLFW;
pub use GameWindowSettings = game_window_settings::GameWindowSettings;
pub use Gl = gl_back_end::Gl;
pub use GlData = gl_back_end::GlData;
pub use AssetStore = asset_store::AssetStore;

pub mod shader_utils;
pub mod game_window;
pub mod keyboard;
pub mod event;
pub mod mouse;

mod game;
mod game_window_sdl2;
mod game_window_glfw;
mod game_window_settings;
mod gl_back_end;
mod asset_store;

