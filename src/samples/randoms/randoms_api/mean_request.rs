use super::{MeanResponse, RandomsApi};
use crate::{samples::Randoms, *};

/// This defines a request to calculate the mean of the contained randoms
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanRequest(pub u64);

/// all requests must be able to provide the id of the pool item that they are targeting
impl IdTargeted for MeanRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

/// ties together the request with a response
impl RequestWithResponse<Randoms> for MeanRequest {
    type Response = MeanResponse;
}

// enable the conversion of the request to the require ThreadRequestResponse
impl From<MeanRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: MeanRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(request)))
    }
}

// enable the conversion from the a ThreadRequestResponse
impl From<ThreadRequestResponse<Randoms>> for MeanRequest {
    fn from(request: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(
            result,
        ))) = request
        else {
            panic!("not expected")
        };
        result
    }
}
