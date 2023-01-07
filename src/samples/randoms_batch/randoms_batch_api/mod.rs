mod randoms_batch_add_request;
mod sum_of_sums_request;
mod sum_of_sums_response;

pub use self::{
    randoms_batch_add_request::RandomsBatchAddRequest, sum_of_sums_request::SumOfSumsRequest,
    sum_of_sums_response::SumOfSumsResponse,
};
use crate::{
    id_targeted::IdTargeted, request_response_2::RequestResponse2, samples::Randoms,
    sender_and_receiver::SenderAndReceiver, thread_request_response::*,
};
use std::fmt::Debug;

use super::RandomsBatch;

#[derive(Debug)]
pub enum RandomsBatchApi<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    SumOfSums(RequestResponse2<RandomsBatch<P>, SumOfSumsRequest>),
}

impl<P> IdTargeted for RandomsBatchApi<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    fn id(&self) -> usize {
        let RandomsBatchApi::SumOfSums(RequestResponse2::Request(sum_of_sum_request)) = self else {
            panic!("id not required to be implemented for responses")
        };
        sum_of_sum_request.id()
    }
}

impl<P> From<ThreadRequestResponse<RandomsBatch<P>>> for RandomsBatchApi<P>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(result) = response else {
                panic!("must be a response to a call to the element")
            };
        result
    }
}

impl<P> From<RandomsBatchApi<P>> for ThreadRequestResponse<RandomsBatch<P>>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(request_response: RandomsBatchApi<P>) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
