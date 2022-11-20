pub mod mean_request;
pub mod mean_response;
pub mod randoms_add_request;
pub mod sum_request;
pub mod sum_response;

use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse,
    thread_request_response::ThreadRequestResponse,
};

use self::{
    mean_request::MeanRequest, mean_response::MeanResponse, sum_request::SumRequest,
    sum_response::SumResponse,
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
