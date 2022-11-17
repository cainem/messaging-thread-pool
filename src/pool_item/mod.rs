pub mod pool_item_api;

use tracing::subscriber::DefaultGuard;
use tracing_appender::non_blocking::WorkerGuard;

use crate::{
    id_targeted::IdTargeted,
    request_response::RequestResponse,
    thread_request_response::{
        add_response::AddResponse, thread_shutdown_response::ThreadShutdownResponse,
        ThreadRequestResponse,
    },
};
use std::fmt::Debug;

use self::pool_item_api::PoolItemApi;

pub trait PoolItem: Debug
where
    Self: Sized,
    Self::Init: IdTargeted,
    Self::Api: PoolItemApi + Debug,
{
    type Init;
    type Api;

    fn process_message(&mut self, request: &Self::Api) -> ThreadRequestResponse<Self>;

    fn id_not_found(request: &Self::Api) -> ThreadRequestResponse<Self> {
        // default behaviour is to panic
        panic!("pool item with id {} not found", request.id());
    }

    /// used for debug only
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn new_pool_item(request: &RequestResponse<Self::Init, AddResponse>) -> Result<Self, ()>;

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

    /// This method provides any required tracing in the elements thread pool threads
    /// This tracing is added when the thread is spawned and remains in place until the thread dies
    #[allow(unused_variables)]
    fn add_pool_thread_tracing(id: usize) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }
}
