use input;

use {
    RenderArgs,
    UpdateArgs,
};
use Event::{ Render, Update, Input };
use events::EventMap;

/// Adds render and update events to input events
#[derive(Clone, PartialEq, Show)]
pub enum Event<I = input::Input> {
    /// Render graphics.
    Render(RenderArgs),
    /// Update the state of the application.
    Update(UpdateArgs),
    /// Input event.
    Input(I),
}

impl<I> EventMap<I> for Event<I> {
    fn render(args: RenderArgs) -> Event<I> { Render(args) }
    fn update(args: UpdateArgs) -> Event<I> { Update(args) }
    fn input(args: I) -> Event<I> { Input(args) }
}

