use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse, samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

use super::RandomsApi;

#[derive(Debug, PartialEq, Eq)]
pub struct MeanRequest(pub usize);

impl IdTargeted for MeanRequest {
    fn id(&self) -> usize {
        self.0
    }
}

impl From<MeanRequest> for ThreadRequestResponse<Randoms> {
    fn from(response: MeanRequest) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(response)))
    }
}

impl From<ThreadRequestResponse<Randoms>> for MeanRequest {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(result))) = response else {
            panic!("not expected")
        };
        result
    }
}
