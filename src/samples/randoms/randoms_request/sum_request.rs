use crate::{
    id_targeted::IdTargeted,
    request_response_pair::RequestResponse,
    samples::{randoms::randoms_api::RandomsApi, Randoms},
    thread_request::ThreadRequest,
    thread_request_response::ThreadRequestResponse,
};

use super::RandomsRequest;

/// This is the message that is sent to request the a given Randoms struct calculates the sum of the random numbers it contains
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumRequest {
    pub id: u64,
}

impl IdTargeted for SumRequest {
    fn id(&self) -> u64 {
        self.id
    }
}

impl From<SumRequest> for ThreadRequest<RandomsRequest> {
    fn from(get_state: SumRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsRequest::Sum(get_state))
    }
}

impl From<SumRequest> for RandomsRequest {
    fn from(request: SumRequest) -> Self {
        RandomsRequest::Sum(request)
    }
}

impl From<SumRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: SumRequest) -> Self {
        ThreadRequestResponse::<Randoms>::CallElement(RandomsApi::Sum(RequestResponse::Request(
            request,
        )))
    }
}
