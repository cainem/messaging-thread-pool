use crate::{id_targeted::IdTargeted, thread_response::ThreadResponse};

use self::{
    mean_response::MeanResponse, randoms_init_response::RandomsInitResponse,
    sum_response::SumResponse,
};

pub mod mean_response;
pub mod randoms_init_response;
pub mod sum_response;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RandomsResponse {
    Mean(MeanResponse),
    Sum(SumResponse),
    Init(RandomsInitResponse),
}

impl IdTargeted for RandomsResponse {
    fn get_id(&self) -> u64 {
        match self {
            RandomsResponse::Mean(do_work) => do_work.get_id(),
            RandomsResponse::Init(init) => init.get_id(),
            RandomsResponse::Sum(get_state) => get_state.get_id(),
        }
    }
}

impl From<RandomsResponse> for ThreadResponse<RandomsResponse> {
    fn from(request: RandomsResponse) -> Self {
        ThreadResponse::ElementResponse(request)
    }
}

impl From<ThreadResponse<RandomsResponse>> for RandomsResponse {
    fn from(thread_response: ThreadResponse<RandomsResponse>) -> Self {
        match thread_response {
            ThreadResponse::ElementResponse(element_response) => element_response,
            _ => panic!("cannot convert"),
        }
    }
}
