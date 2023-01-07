use crate::{
    request_response_2::RequestResponse2,
    samples::{randoms_batch::RandomsBatch, Randoms},
    sender_and_receiver::SenderAndReceiver,
    thread_request_response::ThreadRequestResponse,
};
use std::fmt::Debug;

use super::RandomsBatchApi;

/// This response is returned from a request to calculate the sum of sums of all contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsResponse {
    pub id: usize,
    pub sum_of_sums: u128,
}

impl SumOfSumsResponse {
    pub fn sum_of_sums(&self) -> u128 {
        self.sum_of_sums
    }
}

impl<P> From<SumOfSumsResponse> for ThreadRequestResponse<RandomsBatch<P>>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(response: SumOfSumsResponse) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse2::Response(response),
        ))
    }
}

impl<P> From<ThreadRequestResponse<RandomsBatch<P>>> for SumOfSumsResponse
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(RequestResponse2::Response(response))) = response else {
            panic!("unexpected")
        };
        response
    }
}
