pub mod new_pool_item_error;

use tracing::{event, Level, Subscriber};

use crate::{
    id_targeted::IdTargeted, request_with_response::RequestWithResponse, thread_request_response::*,
};
use std::fmt::Debug;
use std::sync::Arc;

pub use self::new_pool_item_error::NewPoolItemError;

/// This is the trait that needs to be implemented by a struct in order that it can be
/// managed by the thread pool infrastructure
pub trait PoolItem: Debug
where
    Self: Sized,
    Self::Init: Send + IdTargeted + RequestWithResponse<Self, Response = AddResponse>,
    Self::Api: Debug + Send + IdTargeted,
{
    /// This is a struct that defines the message that will initiate a new instance
    /// of the struct within the thread pool
    type Init;
    /// This is the enum that will define that messaging api that can be used to
    /// communicate with instances of the struct
    /// It will be an enum where each variant will define a request/response pair
    /// of structs
    type Api;

    /// This is the function that will define how the struct processes the messages that
    /// it receives.
    /// It will typically consist of a match statement that will discriminate amongst
    /// the various messages type defined in the Api
    fn process_message(&mut self, request: Self::Api) -> ThreadRequestResponse<Self>;

    /// The function called if an item with the specified is not found
    /// The default behaviour is to panic
    fn id_not_found(request: &Self::Api) -> ThreadRequestResponse<Self> {
        // default behaviour is to panic
        event!(Level::ERROR, "pool item with id {} not found", request.id());
        panic!("pool item with id {} not found", request.id());
    }

    /// used for debug only; allows logging to output the name of the type
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// This function defines how a new struct will be created when it receives
    /// The Init message.
    /// It returns the created new instance of the struct
    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError>;

    /// This function is a hook that is called when the pool is shutting down.
    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        Vec::<ThreadShutdownResponse>::default()
    }

    /// This function optionally returns a tracing subscriber that should be used for the
    /// thread pool
    #[allow(unused_variables)]
    fn thread_subscriber(thread_id: usize) -> Option<Box<dyn Subscriber + Send + Sync>> {
        None
    }

    /// This function (optionally) returns a tracing subscriber that should be used solely for tracing a
    /// given pool item
    /// If None then the thread subscriber will be used
    fn pool_item_subscriber(&self) -> Option<Arc<dyn Subscriber + Send + Sync>> {
        None
    }

    /// This method defines the algorithm to be used for routing a given pool item id
    /// to a given pool item thread.
    /// Usually just modding with the thread count is sufficient assuming that ids
    /// are assigned linearly
    fn id_thread_router(id: usize, thread_count: usize) -> usize {
        id % thread_count
    }
}
