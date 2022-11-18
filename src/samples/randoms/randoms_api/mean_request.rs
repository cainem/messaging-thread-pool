use crate::{
    id_targeted::IdTargeted,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
    samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

use super::{RandomsApi, MEAN};

#[derive(Debug, PartialEq, Eq)]
pub struct MeanRequest(pub usize);

impl IdTargeted for MeanRequest {
    fn id(&self) -> usize {
        self.0
    }
}

impl RequestResponseMessage<MEAN, true> for MeanRequest {}

impl From<MeanRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: MeanRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(request)))
    }
}

impl From<ThreadRequestResponse<Randoms>> for MeanRequest {
    fn from(request: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(result))) = request else {
            panic!("not expected")
        };
        result
    }
}
