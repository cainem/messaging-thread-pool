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
    id_targeted::IdTargeted, request_response_2::RequestResponse2,
    thread_request_response::ThreadRequestResponse,
};

use super::Randoms;

/// This enum defines the api used to communicate with the Randoms struct
/// It defines two pairs of messages \
/// One request the calculation of the mean and the other the calculation of the sum
#[derive(Debug, PartialEq)]
pub enum RandomsApi {
    /// a request response pair to handle the calculation of the mean of the contained randoms
    Mean(RequestResponse2<Randoms, MeanRequest>),
    /// a request response pair to handle the calculation of the sum of the contained randoms
    Sum(RequestResponse2<Randoms, SumRequest>),
}

impl IdTargeted for RandomsApi {
    fn id(&self) -> usize {
        match self {
            RandomsApi::Mean(payload) => payload.request().id(),
            RandomsApi::Sum(payload) => payload.request().id(),
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
