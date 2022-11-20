use crate::{
    pool_item::{new_pool_item_error::NewPoolItemError, PoolItem},
    thread_request_response::{
        thread_shutdown_response::ThreadShutdownResponse, ThreadRequestResponse,
    },
};

use super::{
    randoms_batch_api::{
        randoms_batch_add_request::RandomsBatchAddRequest, sum_of_sums_response::SumOfSumsResponse,
        RandomsBatchApi,
    },
    RandomsBatch,
};

impl PoolItem for RandomsBatch {
    type Init = RandomsBatchAddRequest;
    type Api = RandomsBatchApi;

    fn process_message(&mut self, request: &Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsBatchApi::SumOfSums(request) => SumOfSumsResponse {
                id: todo!(),
                sum: todo!(),
            }
            .into(),
        }
    }

    fn new_pool_item(request: &Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(RandomsBatch::new(request))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
