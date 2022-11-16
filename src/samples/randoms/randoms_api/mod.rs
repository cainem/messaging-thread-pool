pub mod mean_request;
pub mod mean_response;
pub mod randoms_init_request;
pub mod randoms_init_response;
pub mod sum_request;
pub mod sum_response;

use crate::{
    id_targeted::IdTargeted, pool_item::pool_item_api::PoolItemApi,
    request_response_pair::RequestResponse, thread_request_response::ThreadRequestResponse,
};

use self::{
    mean_request::MeanRequest, mean_response::MeanResponse, sum_request::SumRequest,
    sum_response::SumResponse,
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
