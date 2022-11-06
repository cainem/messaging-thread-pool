use crate::{id_targeted::IdTargeted, thread_request::ThreadRequest};

use super::RandomsRequest;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InitRequest {
    pub id: u64,
}

impl IdTargeted for InitRequest {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<InitRequest> for ThreadRequest<RandomsRequest> {
    fn from(init_request: InitRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsRequest::Init(init_request))
    }
}

impl From<InitRequest> for RandomsRequest {
    fn from(request: InitRequest) -> Self {
        RandomsRequest::Init(request)
    }
}
