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
}

pub trait PoolItemApi {
    fn is_request(&self) -> bool;
    fn is_response(&self) -> bool {
        !self.is_request()
    }
}
