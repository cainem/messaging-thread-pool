use std::sync::Arc;

use crate::{
    id_provider::sized_id_provider::SizedIdProvider,
    id_targeted::IdTargeted,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
    samples::{randoms_batch::RandomsBatch, Randoms},
    thread_request_response::{ThreadRequestResponse, ADD_POOL_ITEM},
    ThreadPool,
};

/// This is the request that is sent to create a new RandomsBatch
/// It contains a field to configure the size of the contained child thread pool.
/// As the this thread pool is shared it will only ever be used by the first request to create a RandomsBatch
///
/// RandomsBatches will also need to share a common "source of ids" for the Randoms that it will create
#[derive(Debug, Clone)]
pub struct RandomsBatchAddRequest {
    pub id: usize,
    pub number_of_contained_randoms: usize,
    pub id_provider: SizedIdProvider,
    // this thread pool will be shared by all of the Randoms
    pub randoms_thread_pool: Arc<ThreadPool<Randoms>>,
}

impl RandomsBatchAddRequest {
    pub fn id_provider(&self) -> &SizedIdProvider {
        &self.id_provider
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl RequestResponseMessage<ADD_POOL_ITEM, true> for RandomsBatchAddRequest {}

impl IdTargeted for RandomsBatchAddRequest {
    fn id(&self) -> usize {
        self.id
    }
}

impl From<RandomsBatchAddRequest> for ThreadRequestResponse<RandomsBatch> {
    fn from(request: RandomsBatchAddRequest) -> Self {
        ThreadRequestResponse::<RandomsBatch>::AddPoolItem(RequestResponse::Request(request))
    }
}

impl From<ThreadRequestResponse<RandomsBatch>> for RandomsBatchAddRequest {
    fn from(response: ThreadRequestResponse<RandomsBatch>) -> Self {
        let ThreadRequestResponse::AddPoolItem(RequestResponse::Request(result)) = response else {
            panic!("not expected")
        };
        result
    }
}
