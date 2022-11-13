use crate::{
    element::request_response_pair::RequestResponse,
    id_targeted::IdTargeted,
    pool_item::PoolItemApi,
    samples::{
        mean_request::MeanRequest, mean_response::MeanResponse, sum_request::SumRequest,
        sum_response::SumResponse,
    },
    thread_request_response::ThreadRequestResponse,
};

use super::Randoms;

#[derive(Debug)]
pub enum RandomsApi {
    Mean(RequestResponse<MeanRequest, MeanResponse>),
    Sum(RequestResponse<SumRequest, SumResponse>),
}

impl IdTargeted for RandomsApi {
    fn id(&self) -> u64 {
        todo!()
    }
}

impl PoolItemApi for RandomsApi {
    fn is_request(&self) -> bool {
        match self {
            RandomsApi::Mean(payload) => payload.is_request(),
            RandomsApi::Sum(payload) => payload.is_request(),
        }
    }
}

impl From<MeanRequest> for ThreadRequestResponse<Randoms> {
    fn from(request: MeanRequest) -> Self {
        ThreadRequestResponse::CallElement(RandomsApi::Mean(RequestResponse::Request(request)))
    }
}

impl From<ThreadRequestResponse<Randoms>> for MeanResponse {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::CallElement(RandomsApi::Mean(
            RequestResponse::Response(result))) = response else {
                panic!("must be a response to a call to the element")
            };
        result
    }
}
