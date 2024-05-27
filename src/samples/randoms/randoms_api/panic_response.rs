use crate::{samples::Randoms, *};

use super::RandomsApi;

/// The response from a request to calculate the mean
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicResponse(pub u64);

impl From<PanicResponse> for ThreadRequestResponse<Randoms> {
    fn from(response: PanicResponse) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Panic(RequestResponse::Response(
            response,
        )))
    }
}

impl From<ThreadRequestResponse<Randoms>> for PanicResponse {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Panic(RequestResponse::Response(
            result,
        ))) = response
        else {
            panic!("not expected")
        };
        result
    }
}
