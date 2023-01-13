use super::{panic_response::PanicResponse, RandomsApi};
use crate::{samples::Randoms, *};

/// This defines a request to calculate the mean of the contained randoms
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicRequest(pub usize);

/// all requests must be able to provide the id of the pool item that they are targeting
impl IdTargeted for PanicRequest {
    fn id(&self) -> usize {
        self.0
    }
}

/// ties together the request with a response
impl RequestWithResponse<Randoms> for PanicRequest {
    type Response = PanicResponse;
}

// enable the conversion of the request to the require ThreadRequestResponse
impl From<PanicRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: PanicRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Panic(RequestResponse::Request(request)))
    }
}

// enable the conversion from the a ThreadRequestResponse
impl From<ThreadRequestResponse<Randoms>> for PanicRequest {
    fn from(request: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Panic(RequestResponse::Request(result))) = request else {
            panic!("not expected")
        };
        result
    }
}
