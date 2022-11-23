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

// define 2 constant to classify messages
// This allows us to leverage the type system avoid some runtime errors (and replace them with compile time errors)
/// A constant that allows the binding of the mean request and response messages at compile time
pub const MEAN: usize = 0;
/// A constant that allows the binding of the sum request and response messages at compile time
pub const SUM: usize = 1;

/// This enum defines the api used to communicate with the Randoms struct
/// It defines two pairs of messages \
/// One request the calculation of the mean and the other the calculation of the sum
#[derive(Debug, PartialEq, Eq)]
pub enum RandomsApi {
    /// a request response pair to handle the calculation of the mean of the contained randoms
    Mean(RequestResponse<MEAN, MeanRequest, MeanResponse>),
    /// a request response pair to handle the calculation of the sum of the contained randoms
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
                panic!("must be a response to a call to the pool item")
            };
        result
    }
}

impl From<RandomsApi> for ThreadRequestResponse<Randoms> {
    fn from(request_response: RandomsApi) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
