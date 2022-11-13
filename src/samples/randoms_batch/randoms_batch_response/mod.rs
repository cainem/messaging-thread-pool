use crate::id_targeted::IdTargeted;

use self::{
    randoms_batch_init_response::RandomsBatchInitResponse, sum_of_sums_response::SumOfSumsResponse,
};

pub mod randoms_batch_init_response;
pub mod sum_of_sums_response;

/// This enum defines all of the response that can be returned from a RandomsBatch
///
/// Every request is required to return a response
#[derive(Debug)]
pub enum RandomsBatchResponse {
    Init(RandomsBatchInitResponse),
    SumOfSums(SumOfSumsResponse),
}

impl IdTargeted for RandomsBatchResponse {
    fn id(&self) -> u64 {
        match self {
            RandomsBatchResponse::Init(init_request) => init_request.id(),
            RandomsBatchResponse::SumOfSums(sum_of_sums_request) => sum_of_sums_request.id(),
        }
    }
}
