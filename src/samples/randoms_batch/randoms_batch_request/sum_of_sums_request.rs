use crate::{id_targeted::IdTargeted, thread_request::ThreadRequest};

use super::RandomsBatchRequest;

/// This is the message that is sent to request that a given RandomsBatch calculates the sum of all of the
/// sums of its contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsRequest {
    pub id: u64,
}

impl IdTargeted for SumOfSumsRequest {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<SumOfSumsRequest> for ThreadRequest<RandomsBatchRequest> {
    fn from(init_request: SumOfSumsRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsBatchRequest::SumOfSums(init_request))
    }
}

impl From<SumOfSumsRequest> for RandomsBatchRequest {
    fn from(request: SumOfSumsRequest) -> Self {
        RandomsBatchRequest::SumOfSums(request)
    }
}
