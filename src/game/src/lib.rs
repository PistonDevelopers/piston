#![deny(missing_docs)]

//! Simplifed abstractions for games.

use input::{Button, GenericEvent, RenderArgs};
use window::{Window, Size};

/// A simplified application interface for games.
///
/// This is intended to be used with a game manager that sets up window, graphics, audio,
/// handles assets and overall application state such as saving, loading and pausing.
///
/// A game manager might choose to avoid calling some methods,
/// depending on which features that are supported or how the game manager is set up.
///
/// A game might add additional constraints on the window for additional features.
pub trait Game<W> where W: Window {
    /// Returns a list of button configurations and a short description of their usage.
    ///
    /// This is called to retrieve the default button configuration before editing.
    ///
    /// If the game supports multiple input devices, such as keyboard and gamepad,
    /// or for multiple players, the description/button can be repeated.
    ///
    /// The button does not need to match the exact state as when handling events,
    /// e.g. a different gamepad controller can be used when changing button configuration.
    fn buttons(&self) -> Vec<(String, Button)> {vec![]}
    /// Sets new button configuration.
    fn set_buttons(&mut self, _buttons: &[(String, Button)]) {}
    /// A list of fonts to load from assets folder/resource manager.
    ///
    /// This list can vary during the game.
    fn fonts(&self) -> Vec<String> {vec![]}
    /// A list of textures to load from assets folder/resource manager.
    fn textures(&self) -> Vec<String> {vec![]}
    /// A list of sound files to load from assets folder/resource manager.
    fn sounds(&self) -> Vec<String> {vec![]}
    /// A list of music files to load from assets folder/resource manager.
    fn music(&self) -> Vec<String> {vec![]}
    /// Called when loading.
    ///
    /// Offload loading work to other threads.
    /// Return `true` when done loading, `false` when waiting for loading to complete.
    fn loading(&mut self) -> bool {true}
    /// Called when pausing.
    ///
    /// This can be called when the game is interrupted by some external event.
    ///
    /// The game is put into paused state when saving by the game manager.
    fn pause(&mut self) {}
    /// Called when unpaused.
    fn unpaused(&mut self) {}
    /// Render loading screen.
    fn render_loading<C: Canvas>(&mut self, _args: RenderArgs, _draw_size: Size, _canvas: &mut C) {}
    /// Render pausing screen.
    fn render_paused<C: Canvas>(&mut self, _args: RenderArgs, _draw_size: Size, _canvas: &mut C) {}
    /// Render to canvas.
    fn render<C: Canvas>(&mut self, _args: RenderArgs, _draw_size: Size, _canvas: &mut C) {}
    /// Handle events when loading.
    fn event_loading<E: GenericEvent, A: Audio>(
        &mut self,
        _window: &mut W,
        _audio: &mut A,
        _e: &E
    ) {}
    /// Handle events when paused.
    ///
    /// Music is paused while the game is paused.
    fn event_paused<E: GenericEvent>(&mut self, _window: &mut W, _e: &E) {}
    /// Handle event.
    ///
    /// This might handle customized render events, but this requires a custom window wrapper.
    fn event<E: GenericEvent, A: Audio>(
        &mut self,
        _window: &mut W,
        _audio: &mut A,
        _e: &E
    ) {}
    /// Save game state.
    ///
    /// This is called when saving is needed.
    fn save<Wr: std::io::Write>(&self, _w: &mut Wr) -> std::io::Result<()> {Ok(())}
    /// Load game state.
    ///
    /// This is called before `loading` when continuing playing from a previous saved state.
    /// Use this to prepare list of resources.
    fn load<R: std::io::Read>(&mut self, _r: &mut R) -> std::io::Result<()> {Ok(())}
    /// Return `true` to tell game manager to reload game.
    fn reload(&self) -> bool;
}

/// A simplified audio interface.
///
/// Volume settings are handled by the game manager.
pub trait Audio {
    /// Play sound.
    ///
    /// The sound id is the index in the list returned by `Game::sounds`.
    fn play_sound(&mut self, id: usize);
    /// Play music.
    ///
    /// The music id is the index in the list returned by `Game::music`.
    fn play_music(&mut self, id: usize);
    /// Pause music.
    fn pause_music(&mut self);
    /// Returns `true` if the music has ended.
    ///
    /// This might be used to e.g. play a new music track.
    fn music_ended(&self) -> bool;
}

/// A simplified graphics interface.
pub trait Canvas {
    /// Clear the screen.
    fn clear(&mut self);
    /// Draw a line.
    fn line<P: Into<[f64; 2]>>(&mut self, from: P, to: P);
    /// Draw a rectangle.
    fn rectangle<P: Into<[f64; 2]>>(&mut self, from: P, to: P);
    /// Draw rectangle border.
    fn rectangle_border<P: Into<[f64; 2]>>(&mut self, from: P, to: P);
    /// Draw ellipse.
    fn ellipse<P: Into<[f64; 2]>>(&mut self, from: P, to: P);
    /// Draw ellipse border.
    fn ellipse_border<P: Into<[f64; 2]>>(&mut self, from: P, to: P);
    /// Draw filled polygon.
    fn polygon<P: Into<[f64; 2]>>(&mut self, ps: &[P]);
    /// Draw text.
    fn text<P: Into<[f64; 2]>>(&mut self, text: &str, pos: P);
    /// Draw image.
    fn image<P: Into<[f64; 2]>>(&mut self, pos: P);
    /// Set clip region state.
    fn clip(&mut self);
    /// Draw inside clip region.
    fn inside(&mut self);
    /// Draw outside clip region.
    fn outside(&mut self);
    /// Set color.
    fn color<C: Into<[f32; 4]>>(&mut self, color: C);
    /// Set radius;
    fn radius<R: Into<f64>>(&mut self, radius: R);
    /// Set source rectangle in texture for images.
    fn src_rect<P: Into<[f64; 2]>>(&mut self, from: P, to: P);
    /// Set full source rectangle in texture for images.
    fn src_rect_full(&mut self);
    /// Set transform.
    fn transform<M: Into<[[f64; 3]; 2]>>(&mut self, mat: M);
    /// Set font.
    ///
    /// The font id is the index in the list of fonts loaded by `Game::fonts`.
    fn font(&mut self, id: usize);
    /// Set font size.
    fn font_size<S: Into<u32>>(&mut self, font_size: S);
    /// Set texture.
    ///
    /// The texture id is the index in the list of textures loaded by `Game::textures`.
    fn texture(&mut self, id: usize);
    /// Set alpha blend state.
    fn alpha_blend(&mut self);
    /// Set add blend state.
    fn add_blend(&mut self);
    /// Set lighter blend.
    fn lighter_blend(&mut self);
    /// Set multiply blend.
    fn multiply_blend(&mut self);
    /// Set invert blend.
    fn invert_blend(&mut self);
}
