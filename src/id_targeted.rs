use std::fmt::Debug;

/// This trait is implemented by requests that are targeted at an "element" with an id
/// and by the corresponding responses coming back from said element.\
/// This trait is used internally by the thread pool to route requests to the appropriate
/// thread in the thread pool.
pub trait IdTargeted: Send + Debug {
    fn get_id(&self) -> u64;
}
