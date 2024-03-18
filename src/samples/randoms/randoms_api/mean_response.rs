use crate::{samples::Randoms, *};

use super::RandomsApi;

/// The response from a request to calculate the mean
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanResponse {
    pub id: usize,
    pub mean: u128,
}

impl MeanResponse {
    pub fn mean(&self) -> u128 {
        self.mean
    }
}

impl From<MeanResponse> for ThreadRequestResponse<Randoms> {
    fn from(response: MeanResponse) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Response(
            response,
        )))
    }
}

impl From<ThreadRequestResponse<Randoms>> for MeanResponse {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Response(
            result,
        ))) = response
        else {
            panic!("not expected")
        };
        result
    }
}
