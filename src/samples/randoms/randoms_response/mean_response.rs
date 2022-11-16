use crate::{
    id_targeted::IdTargeted, request_response_pair::RequestResponse,
    samples::randoms::randoms_api::RandomsApi,
};

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

impl From<MeanResponse> for RandomsApi {
    fn from(response: MeanResponse) -> Self {
        RandomsApi::Mean(RequestResponse::Response(response))
    }
}
