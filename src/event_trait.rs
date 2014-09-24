
/// A phantom type to get a TypeId for traits.
///
/// To construct a TypeId for an even trait `PressEvent` do the following:
/// `let id = TypeId::of::<EventTrait<PressEvent>>();`
pub enum EventTrait<T> {}
