use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsBatchResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitResponse {
    pub id: u64,
}

impl IdTargeted for InitResponse {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsBatchResponse>> for InitResponse {
    fn from(response: ThreadResponse<RandomsBatchResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsBatchResponse::Init(init)) => init,
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<InitResponse> for RandomsBatchResponse {
    fn from(response: InitResponse) -> Self {
        RandomsBatchResponse::Init(response)
    }
}
