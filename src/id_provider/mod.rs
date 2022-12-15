use std::fmt::Debug;

pub mod id_provider_mutex;
pub mod id_provider_static;
pub mod sized_id_provider;

/// This trait defines a function for getting the next "id" \
/// It is a trait because if the id generator is static different behaviour will be required for test. \
/// Note that only a ref is taken therefore the implementing type needs some sort of interior mutability. \
/// It needs to implement send and sync so that it can be safely passed between threads. \
/// It is included for use in the examples
pub trait IdProvider: Debug + Send + Sync {
    fn peek_next_id(&self) -> usize;
    fn get_next_id(&self) -> usize;
}
