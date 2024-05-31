pub mod new_pool_item_error;

pub use self::new_pool_item_error::NewPoolItemError;
use crate::{
    id_targeted::IdTargeted, request_with_response::RequestWithResponse, thread_request_response::*,
};
use std::fmt::Debug;
use tracing::{event, Level};

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
    /// This is the type that is optionally created when a thread within the thread pool
    /// is started (see thread_start function)
    /// A writeable reference to this is them passed to a pool item each time it is loaded
    /// to "run" (see loading_pool_item function)
    /// The primary motivation behind this is for the implementation of tracing
    /// If this functionality is not required then this can be set to the unit type ()
    type ThreadStartInfo;

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

    /// This function is called once when the message loop to the thread starts up
    /// It gives the implementer the opportunity to set up some shared state
    /// of type `Self::ThreadContext`
    /// The primary reason for adding this hook was to give the implementer the
    /// opportunity to configure tracing/logging for the thread
    fn thread_start() -> Option<Self::ThreadStartInfo> {
        None
    }

    /// This function is called each time that a pool item is loaded into the thread
    /// and is about to start processing a message.
    /// The primary motive here was to provide access to the tracing subscriber in some way
    /// such that tracing can be selectively turned on and off for different pool items
    #[allow(unused_variables)]
    fn pool_item_pre_process(pool_item_id: u64, thread_start_info: &mut Self::ThreadStartInfo) {
        // do nothing by default
    }

    /// This function is called immediately after a pool item has processed a message
    /// The primary motive here was to provide a hook to unload any conditional tracing
    #[allow(unused_variables)]
    fn pool_item_post_process(pool_item_id: u64, thread_start_info: &mut Self::ThreadStartInfo) {
        // do nothing by default
    }

    /// This method defines the algorithm to be used for routing a given pool item id
    /// to a given pool item thread.
    /// Usually just modding with the thread count is sufficient assuming that ids
    /// are assigned linearly
    fn id_thread_router(id: u64, thread_count: usize) -> u64 {
        id % (thread_count as u64)
    }
}
