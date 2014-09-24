use std::intrinsics::TypeId;
use std::any::Any;

/// Used as generic constraint for events.
///
/// Methods should not be called directly.
///
/// An `event_trait_id` is created from `TypeId::of::<Box<Trait>>()`.
/// The implementation need to know the argument type for all event traits.
/// Implementations of `GenericEvent` should be unit tested.
pub trait GenericEvent {
    /// Creates a new event.
    fn from_event(event_trait_id: TypeId, args: &Any) -> Option<Self>;
    /// When correct event type, calls closure with argument.
    fn with_event(&self, event_trait_id: TypeId, f: |&Any|);
}
