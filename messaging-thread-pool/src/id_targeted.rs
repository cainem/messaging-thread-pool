use std::fmt::Debug;

/// This trait is implemented by requests that are targeted at a pool item with an id
/// and by the corresponding responses coming back from said pool item.
/// This trait is used internally by the thread pool to route requests to the appropriate
/// thread in the thread pool.
pub trait IdTargeted: Debug {
    fn id(&self) -> u64;
}

impl IdTargeted for u64 {
    fn id(&self) -> u64 {
        *self
    }
}
