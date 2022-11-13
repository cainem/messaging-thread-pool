use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsResponse;

/// This is the response from a request to calculate the sum of the contained random numbers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumResponse {
    pub id: u64,
    pub sum: u128,
}

impl IdTargeted for SumResponse {
    fn id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsResponse>> for SumResponse {
    fn from(response: ThreadResponse<RandomsResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsResponse::Sum(get_state)) => get_state,
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<SumResponse> for RandomsResponse {
    fn from(response: SumResponse) -> Self {
        RandomsResponse::Sum(response)
    }
}
