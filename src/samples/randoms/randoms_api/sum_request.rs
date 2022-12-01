use crate::{
    id_targeted::IdTargeted,
    request_response::{RequestResponse, RequestResponseMessage},
    samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

use super::{RandomsApi, SUM};

#[derive(Debug, PartialEq, Eq)]
pub struct SumRequest(pub usize);

impl RequestResponseMessage<SUM, true> for SumRequest {}

impl IdTargeted for SumRequest {
    fn id(&self) -> usize {
        self.0
    }
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
