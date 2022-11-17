use crate::{id_provider::sized_id_provider::SizedIdProvider, id_targeted::IdTargeted};

use super::RandomsBatchRequest;

/// This is the request that is sent to create a new RandomsBatch
/// It contains a field to configure the size of the contained child thread pool.
/// As the this thread pool is shared it will only ever be used by the first request to create a RandomsBatch
///
/// RandomsBatches will also need to share a common "source of ids" for the Randoms that it will create
#[derive(Debug, PartialEq, Clone)]
pub struct RandomsBatchInitRequest {
    pub id: usize,
    pub number_of_contained_randoms: u64,
    pub thread_pool_size: usize,
    pub id_provider: SizedIdProvider,
}

impl IdTargeted for RandomsBatchInitRequest {
    fn id(&self) -> usize {
        self.id
    }
}

impl From<RandomsBatchInitRequest> for RandomsBatchRequest {
    fn from(request: RandomsBatchInitRequest) -> Self {
        RandomsBatchRequest::Init(request)
    }
}
