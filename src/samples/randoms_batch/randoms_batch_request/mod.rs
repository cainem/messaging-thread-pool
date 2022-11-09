use crate::id_targeted::IdTargeted;

use self::{
    randoms_batch_init_request::RandomsBatchInitRequest, sum_of_sums_request::SumOfSumsRequest,
};

pub mod randoms_batch_init_request;
pub mod sum_of_sums_request;

/// This defines all of the request that a RandomsBatch can receive.
///
/// It is in essence the api support the the RandomsBatch struct.
#[derive(Debug, PartialEq)]
pub enum RandomsBatchRequest {
    Init(RandomsBatchInitRequest),
    SumOfSums(SumOfSumsRequest),
}

impl IdTargeted for RandomsBatchRequest {
    fn get_id(&self) -> u64 {
        match self {
            RandomsBatchRequest::Init(init_request) => init_request.get_id(),
            RandomsBatchRequest::SumOfSums(sum_of_sums_request) => sum_of_sums_request.get_id(),
        }
    }
}
