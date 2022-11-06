pub mod element_factory;
pub mod element_tracing;
pub mod message_processor;

use crate::{id_targeted::IdTargeted, thread_response::ThreadShutdownResponse};

use self::{
    element_factory::ElementFactory, element_tracing::ElementTracing,
    message_processor::MessageProcessor,
};

/// This is the trait that needs to be implemented by the elements that a PoolThread contains
///
/// It has 2 associated types that correspond to the types of the requests and the responses that can be
/// sent to the element
pub trait Element:
    ElementFactory<Self::Request, Self::Response>
    + MessageProcessor<Self::Request, Self::Response>
    + ElementTracing
{
    type Request: IdTargeted + PartialEq;
    type Response: IdTargeted;

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// if an element is itself using a thread pool, i.e. there is a hierarchy of thread pools
    /// then an element needs to be able to shutdown its contained pool.
    /// Defaults to empty array indicating no child threads
    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        Vec::<ThreadShutdownResponse>::default()
    }
}
