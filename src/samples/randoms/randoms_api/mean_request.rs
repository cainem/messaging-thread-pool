use crate::{
    id_targeted::IdTargeted,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
    samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

use super::{RandomsApi, MEAN};

/// This defines a request to calculate the mean of the contained randoms
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanRequest(pub usize);

/// all requests must be able to provide the id of the pool item that they are targeting
impl IdTargeted for MeanRequest {
    fn id(&self) -> usize {
        self.0
    }
}

// implementing this trait enables the association of a request and a response via
// a constant.
// This helps eliminate a some run time errors, instead promoting them to compile time errors
impl RequestResponseMessage<MEAN, true> for MeanRequest {}

// enable the conversion of the request to the require ThreadRequestResponse
impl From<MeanRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: MeanRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(request)))
    }
}

// enable the conversion from the a ThreadRequestResponse
impl From<ThreadRequestResponse<Randoms>> for MeanRequest {
    fn from(request: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(result))) = request else {
            panic!("not expected")
        };
        result
    }
}
