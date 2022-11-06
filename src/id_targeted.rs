use std::fmt::Debug;

/// This trait is implemented by requests that are targeted at an "element" with an id
/// and by the corresponding responses coming back from said element
pub trait IdTargeted: Send + Debug {
    fn get_id(&self) -> u64;
}
