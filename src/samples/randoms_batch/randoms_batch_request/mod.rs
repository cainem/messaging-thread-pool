use crate::id_targeted::IdTargeted;

use self::{init_request::InitRequest, sum_of_sums_request::SumOfSumsRequest};

pub mod init_request;
pub mod sum_of_sums_request;

#[derive(Debug, PartialEq)]
pub enum RandomsBatchRequest {
    Init(InitRequest),
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
