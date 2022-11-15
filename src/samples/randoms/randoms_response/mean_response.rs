use crate::{
    element::request_response_pair::RequestResponse, id_targeted::IdTargeted,
    samples::randoms::randoms_api::RandomsApi, thread_response::ThreadResponse,
};

use super::RandomsResponse;

/// The response from a request to calculate the mean
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanResponse {
    pub id: u64,
    pub mean: u128,
}

impl IdTargeted for MeanResponse {
    fn id(&self) -> u64 {
        self.id
    }
}

impl From<ThreadResponse<RandomsResponse>> for MeanResponse {
    fn from(response: ThreadResponse<RandomsResponse>) -> Self {
        match response {
            ThreadResponse::ElementResponse(RandomsResponse::Mean(mean)) => mean,
            _ => panic!("cannot unwrap"),
        }
    }
}

impl From<MeanResponse> for RandomsApi {
    fn from(response: MeanResponse) -> Self {
        RandomsApi::Mean(RequestResponse::Response(response))
    }
}

impl From<MeanResponse> for RandomsResponse {
    fn from(response: MeanResponse) -> Self {
        RandomsResponse::Mean(response)
    }
}
