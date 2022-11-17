use crate::{
    id_targeted::IdTargeted,
    request_response::RequestResponse,
    samples::{randoms::randoms_api::RandomsApi, Randoms},
    thread_request_response::ThreadRequestResponse,
};

/// This is the response from a request to calculate the sum of the contained random numbers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumResponse {
    pub id: usize,
    pub sum: u128,
}

impl IdTargeted for SumResponse {
    fn id(&self) -> usize {
        self.id
    }
}

impl From<SumResponse> for ThreadRequestResponse<Randoms> {
    fn from(response: SumResponse) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Sum(RequestResponse::Response(response)))
    }
}

impl From<ThreadRequestResponse<Randoms>> for SumResponse {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Sum(RequestResponse::Response(response))) = response else {
            panic!("unexpected")
        };
        response
    }
}
