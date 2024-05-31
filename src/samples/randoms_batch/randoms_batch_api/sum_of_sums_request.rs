use crate::{
    samples::{Randoms, RandomsBatch},
    *,
};
use std::fmt::Debug;

use super::{RandomsBatchApi, SumOfSumsResponse};

/// This is the message that is sent to request that a given RandomsBatch calculates the sum of all of the
/// sums of its contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsRequest(pub u64);

impl IdTargeted for SumOfSumsRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

/// ties together the request with a response
impl<P> RequestWithResponse<RandomsBatch<P>> for SumOfSumsRequest
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    type Response = SumOfSumsResponse;
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
        let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Request(result),
        )) = request
        else {
            panic!("not expected")
        };
        result
    }
}
