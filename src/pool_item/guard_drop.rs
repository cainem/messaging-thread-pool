/// A trait that is auto implemented for all types that is used to allow for multiple drop guards
/// (or different types) to be returned in a single vec
pub trait GuardDrop: Send + Sync {}

impl<T> GuardDrop for T where T: Send + Sync {}
