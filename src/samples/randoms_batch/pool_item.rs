use crate::{
    id_targeted::IdTargeted,
    pool_item::{new_pool_item_error::NewPoolItemError, PoolItem},
    request_response_2::RequestResponse2,
    samples::Randoms,
    sender_and_receiver::SenderAndReceiver,
    thread_request_response::*,
};
use std::fmt::Debug;

use super::{randoms_batch_api::*, RandomsBatch};

impl<P> PoolItem for RandomsBatch<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    type Init = RandomsBatchAddRequest<P>;
    type Api = RandomsBatchApi<P>;

    fn process_message(&mut self, request: Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsBatchApi::SumOfSums(RequestResponse2::Request(request)) => {
                let id = request.id();
                let sum_of_sums = self.sum_of_sums();
                SumOfSumsResponse { id, sum_of_sums }.into()
            }
            _ => panic!("unexpected"),
        }
    }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(RandomsBatch::new(request))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
