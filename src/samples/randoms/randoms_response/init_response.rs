use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use super::RandomsResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitResponse {
    pub id: u64,
}

impl IdTargeted for InitResponse {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsResponse>> for InitResponse {
    fn from(response: ThreadResponse<RandomsResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsResponse::Init(init)) => init,
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<InitResponse> for RandomsResponse {
    fn from(response: InitResponse) -> Self {
        RandomsResponse::Init(response)
    }
}
