/// Trait for objects that can update their state.
/// Used for animations and time-based state changes.
pub trait Updatable {
    /// Update the internal state of the object.
    /// Called once per frame/update cycle.
    fn update(&mut self);
}
