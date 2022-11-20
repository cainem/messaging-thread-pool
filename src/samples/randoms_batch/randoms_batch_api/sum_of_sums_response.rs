use crate::{
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
    samples::randoms_batch::RandomsBatch,
    thread_request_response::ThreadRequestResponse,
};

use super::{RandomsBatchApi, SUM_OF_SUMS};

/// This response is returned from a request to calculate the sum of sums of all contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsResponse {
    pub id: u64,
    pub sum: u128,
}

impl RequestResponseMessage<SUM_OF_SUMS, false> for SumOfSumsResponse {}

impl From<SumOfSumsResponse> for ThreadRequestResponse<RandomsBatch> {
    fn from(response: SumOfSumsResponse) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Response(response),
        ))
    }
}

impl From<ThreadRequestResponse<RandomsBatch>> for SumOfSumsResponse {
    fn from(response: ThreadRequestResponse<RandomsBatch>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(RequestResponse::Response(response))) = response else {
            panic!("unexpected")
        };
        response
    }
}
