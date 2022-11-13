use crate::{
    element::request_response_pair::RequestResponse, pool_item::PoolItem,
    samples::randoms_init_request::RandomsInitRequest,
    thread_request_response::add_response::AddResponse,
};

use super::{randoms_api::RandomsApi, Randoms};

impl PoolItem for Randoms {
    type Init = RandomsInitRequest;
    type Api = RandomsApi;

    fn process_message(&mut self, request: &Self::Api) -> Self::Api {
        todo!()
    }

    fn new_pool_item(request: &RequestResponse<Self::Init, AddResponse>) -> Result<Self, ()> {
        todo!()
    }
}
