use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    thread_request_response::{
        thread_shutdown_response::ThreadShutdownResponse, ThreadRequestResponse,
    },
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

    fn new_pool_item(request: &Self::Init) -> Result<Self, ()> {
        Ok(Randoms::new(request.0))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
