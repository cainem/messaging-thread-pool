mod mean_request;
mod mean_response;
pub mod panic_request;
pub mod panic_response;
mod randoms_add_request;
mod sum_request;
mod sum_response;

pub use mean_request::MeanRequest;
pub use mean_response::MeanResponse;
pub use randoms_add_request::RandomsAddRequest;
pub use sum_request::SumRequest;
pub use sum_response::SumResponse;

use self::panic_request::PanicRequest;
use super::Randoms;
use crate::*;

/// This enum defines the api used to communicate with the Randoms struct
/// It defines two pairs of messages \
/// One request the calculation of the mean and the other the calculation of the sum
#[derive(Debug, PartialEq)]
pub enum RandomsApi {
    /// a request response pair to handle the calculation of the mean of the contained randoms
    Mean(RequestResponse<Randoms, MeanRequest>),
    /// a request response pair to handle the calculation of the sum of the contained randoms
    Sum(RequestResponse<Randoms, SumRequest>),
    Panic(RequestResponse<Randoms, PanicRequest>),
}

impl IdTargeted for RandomsApi {
    fn id(&self) -> usize {
        match self {
            RandomsApi::Mean(payload) => payload.request().id(),
            RandomsApi::Sum(payload) => payload.request().id(),
            RandomsApi::Panic(payload) => payload.request().id(),
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

#[cfg(test)]
mod tests {
    use crate::{IdTargeted, RequestResponse};

    use super::{panic_request::PanicRequest, MeanRequest, RandomsApi, SumRequest};

    #[test]
    fn returns_expected_ids_for_all_api_variants() {
        let mean_0 = RandomsApi::Mean(RequestResponse::new(MeanRequest(0)));
        let mean_1 = RandomsApi::Mean(RequestResponse::new(MeanRequest(1)));

        let sum_0 = RandomsApi::Sum(RequestResponse::new(SumRequest(0)));
        let sum_1 = RandomsApi::Sum(RequestResponse::new(SumRequest(1)));

        let panic_0 = RandomsApi::Panic(RequestResponse::new(PanicRequest(0)));
        let panic_1 = RandomsApi::Panic(RequestResponse::new(PanicRequest(1)));

        assert_eq!(0, mean_0.id());
        assert_eq!(1, mean_1.id());
        assert_eq!(0, sum_0.id());
        assert_eq!(1, sum_1.id());
        assert_eq!(0, panic_0.id());
        assert_eq!(1, panic_1.id());
    }
}
