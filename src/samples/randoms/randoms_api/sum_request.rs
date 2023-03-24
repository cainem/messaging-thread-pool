use super::{RandomsApi, SumResponse};
use crate::{samples::Randoms, *};

#[derive(Debug, PartialEq, Eq)]
pub struct SumRequest(pub usize);

impl IdTargeted for SumRequest {
    fn id(&self) -> usize {
        self.0
    }
}

/// ties together the request with a response
impl RequestWithResponse<Randoms> for SumRequest {
    type Response = SumResponse;
}

impl From<SumRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: SumRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Sum(RequestResponse::Request(request)))
    }
}

impl From<ThreadRequestResponse<Randoms>> for SumRequest {
    fn from(request: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Sum(RequestResponse::Request(result))) = request else {
            panic!("not expected")
        };
        result
    }
}
