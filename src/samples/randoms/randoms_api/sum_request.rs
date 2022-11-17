use crate::{
    id_targeted::IdTargeted,
    request_response::RequestResponse,
    samples::{randoms::randoms_api::RandomsApi, Randoms},
    thread_request_response::ThreadRequestResponse,
};

/// This is the message that is sent to request the a given Randoms struct calculates the sum of the random numbers it contains
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumRequest {
    pub id: usize,
}

impl IdTargeted for SumRequest {
    fn id(&self) -> usize {
        self.id
    }
}

impl From<SumRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: SumRequest) -> Self {
        ThreadRequestResponse::<Randoms>::MessagePoolItem(RandomsApi::Sum(
            RequestResponse::Request(request),
        ))
    }
}
