use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumResponse {
    pub id: u64,
    pub sum: u128,
}

impl IdTargeted for SumResponse {
    fn get_id(&self) -> u64 {
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
