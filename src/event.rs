use input;

use {
    EventMap,
    IdleArgs,
    RenderArgs,
    AfterRenderArgs,
    UpdateArgs,
};
use Event::{ Idle, Render, AfterRender, Update, Input };

/// Adds render and update events to input events
#[derive(Clone, PartialEq, Debug)]
pub enum Event<I = input::Input> {
    /// Render graphics.
    Render(RenderArgs),
    /// After rendering and swapping buffers.
    AfterRender(AfterRenderArgs),
    /// Update the state of the application.
    Update(UpdateArgs),
    /// Do background tasks that can be done incrementally.
    Idle(IdleArgs),
    /// Input event.
    Input(I),
}

impl<I> EventMap<I> for Event<I> {
    fn render(args: RenderArgs) -> Event<I> { Render(args) }
    fn after_render(args: AfterRenderArgs) -> Event<I> { AfterRender(args) }
    fn update(args: UpdateArgs) -> Event<I> { Update(args) }
    fn idle(args: IdleArgs) -> Event<I> { Idle(args) }
    fn input(args: I) -> Event<I> { Input(args) }
}
