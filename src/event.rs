use input;

use {
    IdleArgs,
    RenderArgs,
    UpdateArgs,
};
use Event::{ Idle, Render, Update, Input };
use events::EventMap;

/// Adds render and update events to input events
#[derive(Clone, PartialEq, Show)]
pub enum Event<I = input::Input> {
    /// Render graphics.
    Render(RenderArgs),
    /// Update the state of the application.
    Update(UpdateArgs),
    /// Do background tasks that can be done incrementally.
    Idle(IdleArgs),
    /// Input event.
    Input(I),
}

impl<I> EventMap<I> for Event<I> {
    fn render(args: RenderArgs) -> Event<I> { Render(args) }
    fn update(args: UpdateArgs) -> Event<I> { Update(args) }
    fn idle(args: IdleArgs) -> Event<I> { Idle(args) }
    fn input(args: I) -> Event<I> { Input(args) }
}
