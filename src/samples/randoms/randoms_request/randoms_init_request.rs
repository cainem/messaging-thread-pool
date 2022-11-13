use crate::{id_targeted::IdTargeted, thread_request::ThreadRequest};

use super::RandomsRequest;

/// This is message that sent to request the creation of a new Randoms struct with the specified id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomsInitRequest {
    pub id: u64,
}

impl IdTargeted for RandomsInitRequest {
    fn id(&self) -> u64 {
        self.id
    }
}

impl From<RandomsInitRequest> for ThreadRequest<RandomsRequest> {
    fn from(init_request: RandomsInitRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsRequest::Init(init_request))
    }
}

impl From<RandomsInitRequest> for RandomsRequest {
    fn from(request: RandomsInitRequest) -> Self {
        RandomsRequest::Init(request)
    }
}
