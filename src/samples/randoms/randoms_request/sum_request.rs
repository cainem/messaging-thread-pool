use crate::{id_targeted::IdTargeted, thread_request::ThreadRequest};

use super::RandomsRequest;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumRequest {
    pub id: u64,
}

impl IdTargeted for SumRequest {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<SumRequest> for ThreadRequest<RandomsRequest> {
    fn from(get_state: SumRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsRequest::Sum(get_state))
    }
}

impl From<SumRequest> for RandomsRequest {
    fn from(request: SumRequest) -> Self {
        RandomsRequest::Sum(request)
    }
}
