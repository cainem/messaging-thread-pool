use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsResponse;

/// The response from a request to create a new Randoms struct within the thread pool
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomsInitResponse {
    pub id: u64,
}

impl IdTargeted for RandomsInitResponse {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsResponse>> for RandomsInitResponse {
    fn from(response: ThreadResponse<RandomsResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsResponse::Init(init)) => init,
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<RandomsInitResponse> for RandomsResponse {
    fn from(response: RandomsInitResponse) -> Self {
        RandomsResponse::Init(response)
    }
}
