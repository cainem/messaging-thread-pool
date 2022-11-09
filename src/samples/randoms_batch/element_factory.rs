use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::{
    element::element_factory::ElementFactory,
    id_provider::IdProvider,
    samples::randoms::{randoms_request, randoms_response, Randoms},
    thread_pool_batcher::*,
    ThreadPool,
};

use super::{
    randoms_batch_request::RandomsBatchRequest,
    randoms_batch_response::{self, RandomsBatchResponse},
    RandomsBatch,
};

static RANDOMS_THREAD_POOL: OnceCell<Arc<ThreadPool<Randoms>>> = OnceCell::new();

/// This is an example of how to implement an element that contains elements that are also in a (child) thread
/// pool.
///
/// Here RandomBatches each contain Randoms.
/// Randoms are required to have the own distinct thread pool from that of RandomBatches so as Randoms and RandomBatches
/// are not able to starve each other
///
/// The Randoms thread pool needs to be Static so that it can be shared between all of the RandomBatches.
/// This doesn't have to be the case. Each RandomBatch could have its own thread pool for Randoms but
/// there are huge reuse benefits from sharing a thread pool
impl ElementFactory<RandomsBatchRequest, RandomsBatchResponse> for RandomsBatch {
    #[inline(always)]
    fn new_element(request: &RandomsBatchRequest) -> (Option<Self>, RandomsBatchResponse) {
        if let RandomsBatchRequest::Init(init_request) = request {
            // lazily create the thread pool that will be used for the internal Randoms and is shared
            // by all of the RandomBatches
            let randoms_thread_pool = RANDOMS_THREAD_POOL.get_or_init(|| {
                Arc::new(ThreadPool::<Randoms>::new(init_request.thread_pool_size))
            });

            // get another reference to the thread pool and downgrade.
            // it is important the Arc only has one reference so that we can shut the threads down when requested
            let cloned = Arc::<ThreadPool<Randoms>>::downgrade(randoms_thread_pool);

            let randoms_thread_pool_batcher = ThreadPoolBatcherConcrete::<Randoms>::new(cloned);

            // create new random batch
            let mut random_batches = RandomsBatch::new_from_init_request(
                init_request.clone(),
                randoms_thread_pool_batcher,
            );

            // create the randoms that are controlled by the RandomBatches
            for _ in 0..init_request.number_of_contained_randoms {
                random_batches.randoms_thread_pool_batcher().batch_for_send(
                    randoms_request::randoms_init_request::RandomsInitRequest {
                        id: random_batches.id_provider.get_next_id(),
                    },
                );
            }
            let responses: Vec<randoms_response::randoms_init_response::RandomsInitResponse> =
                random_batches.randoms_thread_pool_batcher().send_batch();

            // store the ids of the Randoms controlled by this RandomBatches
            responses
                .into_iter()
                .for_each(|e| random_batches.contained_random_ids_mut().push(e.id));

            (
                Some(random_batches),
                randoms_batch_response::randoms_batch_init_response::RandomsBatchInitResponse {
                    id: init_request.id,
                }
                .into(),
            )
        } else {
            panic!("only expecting an init request here");
        }
    }
}
