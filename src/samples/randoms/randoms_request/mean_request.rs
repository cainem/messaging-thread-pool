use crate::{id_targeted::IdTargeted, thread_request::ThreadRequest};

use super::RandomsRequest;

/// This is the message sent to request that the Randoms struct (with the given id) calculates the mean of the random numbers it contains.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MeanRequest {
    pub id: u64,
}

impl IdTargeted for MeanRequest {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<MeanRequest> for RandomsRequest {
    fn from(request: MeanRequest) -> Self {
        RandomsRequest::Mean(request)
    }
}

impl From<MeanRequest> for ThreadRequest<RandomsRequest> {
    fn from(request: MeanRequest) -> Self {
        ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Mean(request))
    }
}
