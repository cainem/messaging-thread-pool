use std::sync::Arc;

use messaging_thread_pool::{
    global_test_scope::global_test_scope,
    id_provider::{id_provider_mutex::IdProviderMutex, sized_id_provider::SizedIdProvider},
    samples::*,
    thread_request_response::*,
    ThreadPool,
};
use tracing::metadata::LevelFilter;

/// This example shows a 2 level thread pool example
///
/// A collection of RandomBatches are assigned to run inside a thread pool
/// Each one of these RandomBatches in turn controls a set of Randoms
/// The Randoms run in their own private thread pool
/// This arrangement means that neither set of objects can starve the other set
///
#[test]
pub fn example_random_batches_() {
    global_test_scope(LevelFilter::OFF);

    // Create a thread pool for RandomsBatch
    // It is the lifetime of this struct that controls the lifetime of all of the pool items that are added
    let randoms_batch_thread_pool = ThreadPool::<RandomsBatch>::new(2);

    // Create another thread pool to be used by the children of the RandomsBatches (which are Randoms)
    // The arrangement here is to have this thread shared by all of the Randoms regardless of which RandomsBatch
    // is their parent. For this reason this thread pool is wrapped in an Arc.
    let randoms_thread_pool = Arc::new(ThreadPool::<Randoms>::new(4));

    // as a shared thread pool will be used for all Randoms it is important that the RandomsBatches share an id provider
    // (the Randoms ids need to be unique across all RandomBatches )
    // this id provider uses a mutex to ensure it provides unique ids
    let id_provider = SizedIdProvider::new(IdProviderMutex::new(0));

    // Create 10 requests to create randoms batches
    // Each RandomsBatch will in turn create 100 Randoms.
    // The thread pool for the Randoms will contain 4 dedicated threads
    // each one will in turn contain 10 randoms that will be distributed across a thread pool with 4 threads

    // this call distributes the work across the thread pool and blocks until all of the work is done
    randoms_batch_thread_pool
        .send_and_receive((0..10).map(|id| RandomsBatchAddRequest {
            id,
            number_of_contained_randoms: 100,
            id_provider: id_provider.clone(),
            randoms_thread_pool: Arc::clone(&randoms_thread_pool),
        }))
        .for_each(|response: AddResponse| assert!(response.success()));

    // now request the "sum of sums" from each RandomBatch by sending a request to each of the RandomsBatches
    // This generates a large amount of work across the 2 thread pools.
    // One thread pool is dedicated to the work of running the RandomsBatches, the other is dedicated to the
    // work of running the Randoms

    // this call distributes the work across the thread pool and blocks until all of the work is done
    let sum_of_sums: Vec<u128> = randoms_batch_thread_pool
        .send_and_receive((0..10).map(|id| SumOfSumsRequest(id)))
        .map(|response: SumOfSumsResponse| response.sum_of_sums())
        .collect();

    dbg!(sum_of_sums);
}
