use crate::{
    request_response::{RequestResponse, RequestResponseMessage},
    samples::{randoms::randoms_api::RandomsApi, Randoms},
    thread_request_response::ThreadRequestResponse,
};

use super::SUM;

/// This is the response from a request to calculate the sum of the contained random numbers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumResponse {
    pub id: usize,
    pub sum: u128,
}

impl SumResponse {
    pub fn sum(&self) -> u128 {
        self.sum
    }
}

impl RequestResponseMessage<SUM, false> for SumResponse {}

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
