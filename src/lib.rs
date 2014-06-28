#![crate_id = "piston"]
#![deny(missing_doc)]
#![warn(dead_code)]

//! A user friendly graphics engine.

extern crate time;

pub use Game = game::Game;

pub use game_iterator::Render;
pub use game_iterator::Update;
pub use game_iterator::KeyPress;
pub use game_iterator::KeyRelease;
pub use game_iterator::MousePress;
pub use game_iterator::MouseRelease;
pub use game_iterator::MouseMove;
pub use game_iterator::MouseRelativeMove;
pub use game_iterator::MouseScroll;

pub use game_iterator::GameEvent;
pub use game_iterator::GameIterator;
pub use game_iterator::GameIteratorSettings;
pub use game_iterator::RenderArgs;
pub use game_iterator::UpdateArgs;
pub use game_iterator::KeyPressArgs;
pub use game_iterator::KeyReleaseArgs;
pub use game_iterator::MousePressArgs;
pub use game_iterator::MouseReleaseArgs;
pub use game_iterator::MouseMoveArgs;
pub use game_iterator::MouseRelativeMoveArgs;
pub use game_iterator::MouseScrollArgs;

pub use game_window::GameWindow;
pub use game_window_settings::GameWindowSettings;
pub use asset_store::AssetStore;

pub mod game_window;
pub mod keyboard;
pub mod event;
pub mod mouse;

mod game;
mod game_iterator;
mod game_window_settings;
mod asset_store;
