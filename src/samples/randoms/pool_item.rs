use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response::RequestResponse,
    thread_request_response::{add_response::AddResponse, ThreadRequestResponse},
};

use super::{
    randoms_api::{
        mean_response::MeanResponse, randoms_add_request::RandomsAddRequest,
        sum_response::SumResponse, RandomsApi,
    },
    Randoms,
};

impl PoolItem for Randoms {
    type Init = RandomsAddRequest;
    type Api = RandomsApi;

    fn process_message(&mut self, request: &Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsApi::Mean(request) => MeanResponse {
                id: request.request().id(),
                mean: self.mean(),
            }
            .into(),
            RandomsApi::Sum(request) => SumResponse {
                id: request.request().id(),
                sum: self.sum(),
            }
            .into(),
        }
    }

    fn new_pool_item(request: &RequestResponse<Self::Init, AddResponse>) -> Result<Self, ()> {
        let RequestResponse::Request(init) = request else {
            panic!("not expected")
        };
        Ok(Randoms::new(init.0))
    }
}
