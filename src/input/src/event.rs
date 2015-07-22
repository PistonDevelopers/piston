use {
    IdleArgs,
    Input,
    RenderArgs,
    AfterRenderArgs,
    UpdateArgs,
};

/// Adds render and update events to input events
#[derive(Clone, PartialEq, Debug)]
pub enum Event<I = Input> {
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
