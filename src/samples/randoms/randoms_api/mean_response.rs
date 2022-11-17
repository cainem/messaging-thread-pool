use crate::{
    id_targeted::IdTargeted,
    request_response::RequestResponse,
    samples::{randoms::randoms_api::RandomsApi, Randoms},
    thread_request_response::ThreadRequestResponse,
};

/// The response from a request to calculate the mean
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanResponse {
    pub id: usize,
    pub mean: u128,
}

impl IdTargeted for MeanResponse {
    fn id(&self) -> usize {
        self.id
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
        let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Response(result))) = response else {
            panic!("not expected")
        };
        result
    }
}
