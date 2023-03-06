use super::{randoms_api::RandomsApi, Randoms};
use crate::{
    guard_drop::GuardDrop,
    samples::{MeanResponse, RandomsAddRequest, SumResponse},
    *,
};

/// The implementation of this trait allows the Randoms struct to be used in the thread pool infrastructure
impl PoolItem for Randoms {
    type Init = RandomsAddRequest;
    type Api = RandomsApi;

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
            RandomsApi::Panic(_request) => panic!("request to panic received"),
        }
    }

    fn add_pool_item_tracing(id: usize) -> Option<Vec<Box<dyn GuardDrop>>> {
        Self::randoms_tracing(id)
    }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(Randoms::new(request.0))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
