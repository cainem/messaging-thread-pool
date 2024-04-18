/// A trait that is auto implemented for all types that is used to allow for multiple drop guards
/// (or different types) to be returned in a single vec
pub trait DropGuard: Send + Sync {}

impl<T> DropGuard for T where T: Send + Sync {}
