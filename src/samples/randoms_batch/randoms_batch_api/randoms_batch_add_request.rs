use crate::{
    id_provider::IdProvider,
    id_targeted::IdTargeted,
    request_response::{RequestResponse, RequestWithResponse},
    samples::{randoms_batch::RandomsBatch, Randoms},
    sender_and_receiver::SenderAndReceiver,
    thread_request_response::{AddResponse, ThreadRequestResponse},
};
use std::fmt::Debug;
use std::sync::Arc;

/// This is the request that is sent to create a new RandomsBatch
/// It contains a field to configure the size of the contained child thread pool.
/// As the this thread pool is shared it will only ever be used by the first request to create a RandomsBatch
///
/// RandomsBatches will also need to share a common "source of ids" for the Randoms that it will create
#[derive(Debug, Clone)]
pub struct RandomsBatchAddRequest<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync,
{
    pub id: usize,
    pub number_of_contained_randoms: usize,
    pub id_provider: Arc<dyn IdProvider>,
    // this thread pool will be shared by all of the Randoms
    pub randoms_thread_pool: Arc<P>,
}

impl<P> RandomsBatchAddRequest<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync,
{
    pub fn id_provider(&self) -> &dyn IdProvider {
        self.id_provider.as_ref()
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<P> IdTargeted for RandomsBatchAddRequest<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    fn id(&self) -> usize {
        self.id
    }
}

impl<P> RequestWithResponse<RandomsBatch<P>> for RandomsBatchAddRequest<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    type Response = AddResponse;
}

impl<P> From<RandomsBatchAddRequest<P>> for ThreadRequestResponse<RandomsBatch<P>>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    fn from(request: RandomsBatchAddRequest<P>) -> Self {
        ThreadRequestResponse::<RandomsBatch<P>>::AddPoolItem(RequestResponse::Request(request))
    }
}

impl<P> From<ThreadRequestResponse<RandomsBatch<P>>> for RandomsBatchAddRequest<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::AddPoolItem(RequestResponse::Request(result)) = response else {
            panic!("not expected")
        };
        result
    }
}
