mod mean_request;
mod mean_response;
mod randoms_add_request;
mod sum_request;
mod sum_response;

pub use mean_request::MeanRequest;
pub use mean_response::MeanResponse;
pub use randoms_add_request::RandomsAddRequest;
pub use sum_request::SumRequest;
pub use sum_response::SumResponse;

use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse,
    thread_request_response::ThreadRequestResponse,
};

use super::Randoms;

/// define 2 constant to classify messages
/// This allows us to leverage the type system avoid some runtime errors (and replace them with compile time errors)
pub const MEAN: usize = 0;
pub const SUM: usize = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum RandomsApi {
    Mean(RequestResponse<MEAN, MeanRequest, MeanResponse>),
    Sum(RequestResponse<SUM, SumRequest, SumResponse>),
}

impl IdTargeted for RandomsApi {
    fn id(&self) -> usize {
        match self {
            RandomsApi::Mean(payload) => payload.id(),
            RandomsApi::Sum(payload) => payload.id(),
        }
    }
}

impl From<ThreadRequestResponse<Randoms>> for RandomsApi {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(result) = response else {
                panic!("must be a response to a call to the element")
            };
        result
    }
}

impl From<RandomsApi> for ThreadRequestResponse<Randoms> {
    fn from(request_response: RandomsApi) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
