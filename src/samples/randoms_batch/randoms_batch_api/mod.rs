mod randoms_batch_add_request;
mod sum_of_sums_request;
mod sum_of_sums_response;

pub use self::{
    randoms_batch_add_request::RandomsBatchAddRequest, sum_of_sums_request::SumOfSumsRequest,
    sum_of_sums_response::SumOfSumsResponse,
};
use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse, samples::Randoms,
    sender_and_receiver::SenderAndReceiver, thread_request_response::*,
};
use std::fmt::Debug;

use super::RandomsBatch;

/// define 2 constant to classify messages
/// This allows us to leverage the type system avoid some runtime errors (and replace them with compile time errors)
pub const SUM_OF_SUMS: usize = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum RandomsBatchApi {
    SumOfSums(RequestResponse<SUM_OF_SUMS, SumOfSumsRequest, SumOfSumsResponse>),
}

impl IdTargeted for RandomsBatchApi {
    fn id(&self) -> usize {
        let RandomsBatchApi::SumOfSums(RequestResponse::Request(sum_of_sum_request)) = self else {
            panic!("id not required to be implemented for responses")
        };
        sum_of_sum_request.id()
    }
}

impl<P> From<ThreadRequestResponse<RandomsBatch<P>>> for RandomsBatchApi
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

impl<P> From<RandomsBatchApi> for ThreadRequestResponse<RandomsBatch<P>>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(request_response: RandomsBatchApi) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
