mod element_factory;
pub mod message_processor;
pub mod randoms_batch_request;
pub mod randoms_batch_response;

use self::{
    randoms_batch_request::{
        randoms_batch_init_request::RandomsBatchInitRequest, RandomsBatchRequest,
    },
    randoms_batch_response::RandomsBatchResponse,
};
use crate::{
    element::{element_tracing::ElementTracing, Element},
    id_provider::sized_id_provider::SizedIdProvider,
    samples::randoms::{
        randoms_request::sum_request::SumRequest, randoms_response::sum_response::SumResponse,
        Randoms,
    },
    thread_pool_batcher::ThreadPoolBatcherConcrete,
    thread_response::ThreadShutdownResponse,
};

/// An example of an element that contains a child thread pool
///
/// RandomsBatches and Randoms form a hierarchy.
/// A RandomsBatch contains many Randoms.
///
/// RandomsBatches are managed by a one thread pool and internally they have a collection of Randoms which
/// are managed in a separate "child" thread pool
/// In this example all of the Randoms share a single thread pool regardless of which RandomsBatch created them
///
/// For this reason the RandomsBatches need to share an id_provider which provides globally unique ids
/// (ids, must be unique across the thread pool for obvious reasons)
#[derive(Debug)]
pub struct RandomsBatch {
    pub id: u64,
    pub contained_random_ids: Vec<u64>,
    pub randoms_thread_pool_batcher: ThreadPoolBatcherConcrete<Randoms>,
    pub id_provider: SizedIdProvider,
}

impl RandomsBatch {
    pub fn new_from_init_request(
        init_request: RandomsBatchInitRequest,
        randoms_thread_pool_batcher: ThreadPoolBatcherConcrete<Randoms>,
    ) -> Self {
        let RandomsBatchInitRequest {
            id,
            number_of_contained_randoms: _,
            thread_pool_size: _,
            id_provider,
        } = init_request;
        Self {
            id,
            contained_random_ids: vec![],
            randoms_thread_pool_batcher,
            id_provider,
        }
    }

    pub fn sum_of_sums(&self) -> u128 {
        // to get the sum of sums need to message the controls Randoms to get their sums
        // and then add them all up
        for contained_id in self.contained_random_ids.iter() {
            self.randoms_thread_pool_batcher
                .batch_for_send(SumRequest { id: *contained_id });
        }
        let sum_of_sums_responses: Vec<SumResponse> = self.randoms_thread_pool_batcher.send_batch();

        sum_of_sums_responses.iter().map(|e| e.sum).sum()
    }

    pub fn contained_random_ids_mut(&mut self) -> &mut Vec<u64> {
        &mut self.contained_random_ids
    }

    pub fn randoms_thread_pool_batcher(&self) -> &ThreadPoolBatcherConcrete<Randoms> {
        &self.randoms_thread_pool_batcher
    }
}

impl ElementTracing for RandomsBatch {}

impl Element for RandomsBatch {
    type Request = RandomsBatchRequest;
    type Response = RandomsBatchResponse;

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        self.randoms_thread_pool_batcher().shutdown_pool()
    }
}
