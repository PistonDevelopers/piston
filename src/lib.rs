#![crate_id = "piston"]
#![deny(missing_doc)]
#![warn(dead_code)]

//! A user friendly graphics engine.
#[cfg(glfw)]
extern crate glfw;
#[cfg(sdl2)]
extern crate sdl2;
#[cfg(glfw)]
#[cfg(sdl2)]
extern crate gl;
extern crate time;
extern crate graphics;
extern crate log;
extern crate collections;
extern crate debug;

pub use Game = game::Game;

pub use Render = game_iterator::Render;
pub use Update = game_iterator::Update;
pub use KeyPress = game_iterator::KeyPress;
pub use KeyRelease = game_iterator::KeyRelease;
pub use MousePress = game_iterator::MousePress;
pub use MouseRelease = game_iterator::MouseRelease;
pub use MouseMove = game_iterator::MouseMove;
pub use MouseRelativeMove = game_iterator::MouseRelativeMove;

pub use GameEvent = game_iterator::GameEvent;
pub use GameIterator = game_iterator::GameIterator;
pub use GameIteratorSettings = game_iterator::GameIteratorSettings;
pub use RenderArgs = game_iterator::RenderArgs;
pub use UpdateArgs = game_iterator::UpdateArgs;
pub use KeyPressArgs = game_iterator::KeyPressArgs;
pub use KeyReleaseArgs = game_iterator::KeyReleaseArgs;
pub use MousePressArgs = game_iterator::MousePressArgs;
pub use MouseReleaseArgs = game_iterator::MouseReleaseArgs;
pub use MouseMoveArgs = game_iterator::MouseMoveArgs;
pub use MouseRelativeMoveArgs = game_iterator::MouseRelativeMoveArgs;

pub use GameWindow = game_window::GameWindow;
#[cfg(sdl2)]
pub use GameWindowSDL2 = game_window_sdl2::GameWindowSDL2;
#[cfg(glfw)]
pub use GameWindowGLFW = game_window_glfw::GameWindowGLFW;
pub use GameWindowSettings = game_window_settings::GameWindowSettings;
pub use AssetStore = asset_store::AssetStore;

pub mod game_window;
pub mod keyboard;
pub mod event;
pub mod mouse;

mod game;
mod game_iterator;
mod game_window_settings;
mod asset_store;

#[cfg(sdl2)]
mod game_window_sdl2;
#[cfg(glfw)]
mod game_window_glfw;

