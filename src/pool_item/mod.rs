use tracing::subscriber::DefaultGuard;
use tracing_appender::non_blocking::WorkerGuard;

use crate::{
    element::request_response_pair::RequestResponse,
    id_targeted::IdTargeted,
    thread_request_response::{
        add_response::AddResponse, thread_shutdown_response::ThreadShutdownResponse,
    },
};
use std::fmt::Debug;

pub trait PoolItem: Debug
where
    Self: Sized,
    Self::Init: IdTargeted,
    Self::Api: IdTargeted + PoolItemApi,
{
    type Init;
    type Api;

    fn process_message(&mut self, request: &Self::Api) -> Self::Api;

    fn id_not_found(request: &Self::Api) -> Self::Api {
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
    fn add_element_request_tracing(id: u64) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }

    /// This method provides any required tracing in the elements thread pool threads
    /// This tracing is added when the thread is spawned and remains in place until the thread dies
    #[allow(unused_variables)]
    fn add_pool_thread_tracing(id: u64) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }
}

pub trait PoolItemApi {
    fn is_request(&self) -> bool;
    fn is_response(&self) -> bool {
        !self.is_request()
    }
}
