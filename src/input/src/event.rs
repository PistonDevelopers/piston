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

impl<I> From<RenderArgs> for Event<I> {
    fn from(args: RenderArgs) -> Self {
        Event::Render(args)
    }
}

impl<I> From<AfterRenderArgs> for Event<I> {
    fn from(args: AfterRenderArgs) -> Self {
        Event::AfterRender(args)
    }
}

impl<I> From<UpdateArgs> for Event<I> {
    fn from(args: UpdateArgs) -> Self {
        Event::Update(args)
    }
}

impl<I> From<IdleArgs> for Event<I> {
    fn from(args: IdleArgs) -> Self {
        Event::Idle(args)
    }
}

impl From<Input> for Event<Input> {
    fn from(input: Input) -> Self {
        Event::Input(input)
    }
}
