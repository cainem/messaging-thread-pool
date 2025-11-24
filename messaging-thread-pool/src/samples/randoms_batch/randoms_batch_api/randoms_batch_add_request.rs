use crate::id_provider::IdProvider;
use crate::samples::RandomsBatch;
use crate::{AddResponse, IdTargeted, RequestResponse, RequestWithResponse, ThreadRequestResponse};
use std::fmt::Debug;
use std::sync::Arc;

use super::InnerThreadPool;

/// This is the request that is sent to create a new RandomsBatch
/// It contains a field to configure the size of the contained child thread pool.
/// As the this thread pool is shared it will only ever be used by the first request to create a RandomsBatch
///
/// RandomsBatches will also need to share a common "source of ids" for the Randoms that it will create
#[derive(Debug, Clone)]
pub struct RandomsBatchAddRequest<P: InnerThreadPool> {
    pub id: u64,
    pub number_of_contained_randoms: usize,
    pub id_provider: Arc<dyn IdProvider>,
    // this thread pool will be shared by all of the Randoms
    pub randoms_thread_pool: Arc<P::ThreadPool>,
}

impl<P: InnerThreadPool> RandomsBatchAddRequest<P> {
    pub fn id_provider(&self) -> &dyn IdProvider {
        self.id_provider.as_ref()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl<P: InnerThreadPool> IdTargeted for RandomsBatchAddRequest<P> {
    fn id(&self) -> u64 {
        self.id
    }
}

impl<P: InnerThreadPool> RequestWithResponse<RandomsBatch<P>> for RandomsBatchAddRequest<P> {
    type Response = AddResponse;
}

impl<P: InnerThreadPool> From<RandomsBatchAddRequest<P>>
    for ThreadRequestResponse<RandomsBatch<P>>
{
    fn from(request: RandomsBatchAddRequest<P>) -> Self {
        ThreadRequestResponse::<RandomsBatch<P>>::AddPoolItem(RequestResponse::Request(request))
    }
}

impl<P: InnerThreadPool> From<ThreadRequestResponse<RandomsBatch<P>>>
    for RandomsBatchAddRequest<P>
{
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::AddPoolItem(RequestResponse::Request(result)) = response else {
            panic!("not expected")
        };
        result
    }
}
