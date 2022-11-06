use crate::id_targeted::IdTargeted;

use self::{init_response::InitResponse, sum_of_sums_response::SumOfSumsResponse};

pub mod init_response;
pub mod sum_of_sums_response;

#[derive(Debug)]
pub enum RandomsBatchResponse {
    Init(InitResponse),
    SumOfSums(SumOfSumsResponse),
}

impl IdTargeted for RandomsBatchResponse {
    fn get_id(&self) -> u64 {
        match self {
            RandomsBatchResponse::Init(init_request) => init_request.get_id(),
            RandomsBatchResponse::SumOfSums(sum_of_sums_request) => sum_of_sums_request.get_id(),
        }
    }
}
