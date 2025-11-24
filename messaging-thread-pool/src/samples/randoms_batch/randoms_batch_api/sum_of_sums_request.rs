use crate::{samples::RandomsBatch, *};
use std::fmt::Debug;

use super::{InnerThreadPool, RandomsBatchApi, SumOfSumsResponse};

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
impl<P: InnerThreadPool> RequestWithResponse<RandomsBatch<P>> for SumOfSumsRequest {
    type Response = SumOfSumsResponse;
}

impl<P: InnerThreadPool> From<SumOfSumsRequest> for ThreadRequestResponse<RandomsBatch<P>> {
    fn from(request: SumOfSumsRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Request(request),
        ))
    }
}

impl<P: InnerThreadPool> From<ThreadRequestResponse<RandomsBatch<P>>> for SumOfSumsRequest {
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
