pub mod pool_item;
pub mod randoms_batch_api;

use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::{
    id_provider::sized_id_provider::SizedIdProvider, samples::randoms::Randoms, ThreadPool,
};

use super::RandomsBatchAddRequest;

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
    pub id: usize,
    pub contained_random_ids: Vec<usize>,
    //pub randoms_thread_pool_batcher: ThreadPoolBatcherConcrete<Randoms>,
    pub id_provider: SizedIdProvider,
    pub randoms_thread_pool: OnceCell<Arc<ThreadPool<Randoms>>>,
}

impl RandomsBatch {
    pub fn new(add_request: &RandomsBatchAddRequest) -> Self {
        Self {
            id: add_request.id,
            contained_random_ids: vec![],
            id_provider: add_request.id_provider.clone(),
            randoms_thread_pool: OnceCell::new(),
        }
    }

    pub fn randoms_thread_pool(&self) -> &ThreadPool<Randoms> {
        self.randoms_thread_pool
            .get_or_init(|| Arc::new(ThreadPool::<Randoms>::new(1)))
    }

    pub fn sum_of_sums(&self) -> u128 {
        // to get the sum of sums need to message the controls Randoms to get their sums
        // and then add them all up
        // for contained_id in self.contained_random_ids.iter() {
        //     self.randoms_thread_pool_batcher
        //         .batch_for_send(SumRequest { id: *contained_id });
        // }
        // let sum_of_sums_responses: Vec<SumResponse> = self.randoms_thread_pool_batcher.send_batch();

        // sum_of_sums_responses.iter().map(|e| e.sum).sum()
        todo!()
    }

    pub fn contained_random_ids_mut(&mut self) -> &mut Vec<usize> {
        &mut self.contained_random_ids
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
