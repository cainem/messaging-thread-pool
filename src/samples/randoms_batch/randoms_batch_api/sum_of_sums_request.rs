use crate::{
    id_targeted::IdTargeted,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
    samples::randoms_batch::RandomsBatch,
    thread_request_response::ThreadRequestResponse,
};

use super::{RandomsBatchApi, SUM_OF_SUMS};

/// This is the message that is sent to request that a given RandomsBatch calculates the sum of all of the
/// sums of its contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsRequest(pub usize);

impl RequestResponseMessage<SUM_OF_SUMS, true> for SumOfSumsRequest {}

impl IdTargeted for SumOfSumsRequest {
    fn id(&self) -> usize {
        self.0
    }
}

impl From<SumOfSumsRequest> for ThreadRequestResponse<RandomsBatch> {
    fn from(request: SumOfSumsRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Request(request),
        ))
    }
}

impl From<ThreadRequestResponse<RandomsBatch>> for SumOfSumsRequest {
    fn from(request: ThreadRequestResponse<RandomsBatch>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(RequestResponse::Request(result))) = request else {
            panic!("not expected")
        };
        result
    }
}
