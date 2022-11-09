use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsBatchResponse;

/// This response is returned from a request to calculate the sum of sums of all contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsResponse {
    pub id: u64,
    pub sum: u128,
}

impl IdTargeted for SumOfSumsResponse {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsBatchResponse>> for SumOfSumsResponse {
    fn from(response: ThreadResponse<RandomsBatchResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsBatchResponse::SumOfSums(sum_of_sums)) => {
                sum_of_sums
            }
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<SumOfSumsResponse> for RandomsBatchResponse {
    fn from(response: SumOfSumsResponse) -> Self {
        RandomsBatchResponse::SumOfSums(response)
    }
}
