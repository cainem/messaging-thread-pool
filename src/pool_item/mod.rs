pub mod new_pool_item_error;

use tracing::{event, subscriber::DefaultGuard, Level};
use tracing_appender::non_blocking::WorkerGuard;

use crate::{
    id_targeted::IdTargeted, request_response::request_response_message::RequestResponseMessage,
    thread_request_response::*,
};
use std::fmt::Debug;

use self::new_pool_item_error::NewPoolItemError;

/// This is the trait that needs to be implemented by a struct in order that it can be
/// managed by the thread pool infrastructure
pub trait PoolItem: Debug
where
    Self: Sized,
    Self::Init: RequestResponseMessage<ADD_POOL_ITEM, true> + IdTargeted,
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
    fn process_message(&mut self, request: &Self::Api) -> ThreadRequestResponse<Self>;

    /// The function called if an item with the specified is not found
    /// The default behaviour is to panic
    fn id_not_found(request: &Self::Api) -> ThreadRequestResponse<Self> {
        // default behaviour is to panic
        event!(Level::ERROR, "pool item with id {} not found", request.id());
        panic!("pool item with id {} not found", request.id());
    }

    /// used for debug only; allows logging to output the name of the type
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// This function defines how a new struct will be created when it receives
    /// The Init message.
    /// It returns the created new instance of the struct
    fn new_pool_item(request: &Self::Init) -> Result<Self, NewPoolItemError>;

    /// This function is a hook that is called when the pool is shutting down.
    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        Vec::<ThreadShutdownResponse>::default()
    }

    /// This method is called to optionally add tracing before each message is processed.
    /// The tracing is removed once the message is processed.
    /// If the tracing is being written to a file it is important that the file is not truncated
    #[allow(unused_variables)]
    fn add_element_request_tracing(id: usize) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }

    /// This method provides any required tracing in the pool items thread pool threads
    /// This tracing is added when the thread is spawned and remains in place until the thread dies
    #[allow(unused_variables)]
    fn add_pool_thread_tracing(id: usize) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }
}
