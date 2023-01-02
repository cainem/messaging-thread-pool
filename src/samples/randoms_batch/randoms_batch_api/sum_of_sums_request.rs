use crate::{
    id_targeted::IdTargeted,
    request_response::{RequestResponse, RequestResponseMessage},
    samples::{randoms_batch::RandomsBatch, Randoms},
    sender_and_receiver::SenderAndReceiver,
    thread_request_response::ThreadRequestResponse,
};
use std::fmt::Debug;

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

impl<P> From<SumOfSumsRequest> for ThreadRequestResponse<RandomsBatch<P>>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(request: SumOfSumsRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Request(request),
        ))
    }
}

impl<P> From<ThreadRequestResponse<RandomsBatch<P>>> for SumOfSumsRequest
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(request: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(RequestResponse::Request(result))) = request else {
            panic!("not expected")
        };
        result
    }
}
