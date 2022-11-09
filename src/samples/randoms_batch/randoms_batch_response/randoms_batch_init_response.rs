use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsBatchResponse;

/// This response is returned from a request to create a new RandomBatch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomsBatchInitResponse {
    pub id: u64,
}

impl IdTargeted for RandomsBatchInitResponse {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsBatchResponse>> for RandomsBatchInitResponse {
    fn from(response: ThreadResponse<RandomsBatchResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsBatchResponse::Init(init)) => init,
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<RandomsBatchInitResponse> for RandomsBatchResponse {
    fn from(response: RandomsBatchInitResponse) -> Self {
        RandomsBatchResponse::Init(response)
    }
}
