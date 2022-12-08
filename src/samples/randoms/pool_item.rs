use crate::{
    id_targeted::IdTargeted,
    pool_item::{new_pool_item_error::NewPoolItemError, PoolItem},
    samples::{MeanResponse, RandomsAddRequest, SumResponse},
    thread_request_response::*,
};

use super::{randoms_api::RandomsApi, Randoms};

/// The implementation of this trait allows the Randoms struct to be used in the thread pool infrastructure
impl PoolItem for Randoms {
    type Init = RandomsAddRequest;
    type Api = RandomsApi;

    /// here
    fn process_message(&mut self, request: Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsApi::Mean(request) => MeanResponse {
                id: request.id(),
                mean: self.mean(),
            }
            .into(),
            RandomsApi::Sum(request) => SumResponse {
                id: request.id(),
                sum: self.sum(),
            }
            .into(),
        }
    }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(Randoms::new(request.0))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
