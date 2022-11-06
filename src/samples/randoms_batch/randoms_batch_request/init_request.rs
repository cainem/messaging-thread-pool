use crate::{
    id_provider::sized_id_provider::SizedIdProvider, id_targeted::IdTargeted,
    thread_request::ThreadRequest,
};

use super::RandomsBatchRequest;

#[derive(Debug, PartialEq, Clone)]
pub struct InitRequest {
    pub id: u64,
    pub number_of_contained_randoms: u64,
    pub thread_pool_size: usize,
    pub id_provider: SizedIdProvider,
}

impl IdTargeted for InitRequest {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<InitRequest> for ThreadRequest<RandomsBatchRequest> {
    fn from(init_request: InitRequest) -> Self {
        ThreadRequest::ElementRequest(RandomsBatchRequest::Init(init_request))
    }
}

impl From<InitRequest> for RandomsBatchRequest {
    fn from(request: InitRequest) -> Self {
        RandomsBatchRequest::Init(request)
    }
}
