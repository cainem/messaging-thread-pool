use crate::id_targeted::IdTargeted;

use super::RandomsBatchRequest;

/// This is the message that is sent to request that a given RandomsBatch calculates the sum of all of the
/// sums of its contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsRequest {
    pub id: usize,
}

impl IdTargeted for SumOfSumsRequest {
    fn id(&self) -> usize {
        self.id
    }
}

impl From<SumOfSumsRequest> for RandomsBatchRequest {
    fn from(request: SumOfSumsRequest) -> Self {
        RandomsBatchRequest::SumOfSums(request)
    }
}
