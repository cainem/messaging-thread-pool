mod mean_request;
mod mean_response;
pub mod panic_request;
pub mod panic_response;
mod randoms_add_request;
mod sum_request;
mod sum_response;

pub use mean_request::MeanRequest;
pub use mean_response::MeanResponse;
use panic_response::PanicResponse;
pub use randoms_add_request::RandomsAddRequest;
pub use sum_request::SumRequest;
pub use sum_response::SumResponse;

use self::panic_request::PanicRequest;
use super::Randoms;
use crate::*;

// implement Randoms using the api_specification macro
// This generates the RandomApi enum as well as generating all of the code
// to perform the mappings of the various request/response structs in and
// out of the ThreadRequestResponse enum.
// The other sample, RandomsBatch, does not use the macro and demonstrates the
// code that would have to be written if it is not used.
api_specification!(pool_item: Randoms, api_name: RandomsApi, add_request: RandomsAddRequest,
calls: [
    { call_name: Mean, request: MeanRequest, response: MeanResponse },
    { call_name: Sum, request: SumRequest, response: SumResponse },
    { call_name: Panic, request: PanicRequest, response: PanicResponse },
    ]);

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
