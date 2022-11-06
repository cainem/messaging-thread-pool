use crate::{
    id_provider::sized_id_provider::SizedIdProvider, id_targeted::IdTargeted,
    thread_request::ThreadRequest,
};

use super::RandomsBatchRequest;

#[derive(Debug, PartialEq, Clone)]
pub struct RandomsBatchInitRequest {
    pub id: u64,
    pub number_of_contained_randoms: u64,
    pub thread_pool_size: usize,
    pub id_provider: SizedIdProvider,
}

impl IdTargeted for RandomsBatchInitRequest {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<RandomsBatchInitRequest> for ThreadRequest<RandomsBatchRequest> {
    fn from(init_request: RandomsBatchInitRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsBatchRequest::Init(init_request))
    }
}

impl From<RandomsBatchInitRequest> for RandomsBatchRequest {
    fn from(request: RandomsBatchInitRequest) -> Self {
        RandomsBatchRequest::Init(request)
    }
}
