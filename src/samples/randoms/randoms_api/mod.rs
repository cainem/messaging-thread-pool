pub mod mean_request;
pub mod mean_response;
pub mod randoms_add_request;
pub mod sum_request;
pub mod sum_response;

use crate::{
    id_targeted::IdTargeted, pool_item::pool_item_api::PoolItemApi,
    request_response::RequestResponse, thread_request_response::ThreadRequestResponse,
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
    fn id(&self) -> usize {
        match self {
            RandomsApi::Mean(payload) => payload.id(),
            RandomsApi::Sum(payload) => payload.id(),
        }
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

impl From<ThreadRequestResponse<Randoms>> for RandomsApi {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(result) = response else {
                panic!("must be a response to a call to the element")
            };
        result
    }
}

impl From<RandomsApi> for ThreadRequestResponse<Randoms> {
    fn from(request_response: RandomsApi) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
