use crate::{
    id_targeted::IdTargeted,
    pool_item::{new_pool_item_error::NewPoolItemError, PoolItem},
    thread_request_response::*,
};

use super::{randoms_batch_api::*, RandomsBatch};

impl PoolItem for RandomsBatch {
    type Init = RandomsBatchAddRequest;
    type Api = RandomsBatchApi;

    fn process_message(&mut self, request: &Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsBatchApi::SumOfSums(request) => {
                let id = request.id();
                let sum_of_sums = self.sum_of_sums();
                SumOfSumsResponse { id, sum_of_sums }.into()
            }
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
