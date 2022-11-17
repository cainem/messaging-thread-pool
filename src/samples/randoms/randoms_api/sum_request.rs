use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse, samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

use super::RandomsApi;

#[derive(Debug, PartialEq, Eq)]
pub struct SumRequest(pub usize);

impl IdTargeted for SumRequest {
    fn id(&self) -> usize {
        self.0
    }
}

impl From<SumRequest> for ThreadRequestResponse<Randoms> {
    fn from(response: SumRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Sum(RequestResponse::Request(response)))
    }
}

impl From<ThreadRequestResponse<Randoms>> for SumRequest {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Sum(RequestResponse::Request(result))) = response else {
            panic!("not expected")
        };
        result
    }
}
