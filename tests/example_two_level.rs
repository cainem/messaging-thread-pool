use std::sync::Arc;

use messaging_thread_pool::{
    id_provider::{id_provider_mutex::IdProviderMutex, sized_id_provider::SizedIdProvider},
    samples::*,
    thread_pool_batcher::ThreadPoolBatcherConcrete,
    ThreadPool,
};

/// This example shows a 2 level thread pool example
///
/// A collection of RandomBatches are assigned to run inside a thread pool
/// Each one of these RandomBatches in turn controls a set of Randoms
/// The Randoms run in their own private thread pool
/// This arrangement means that neither set of objects can starve the other set
///
#[test]
pub fn example_random_batches_() {
    // Create a thread pool for RandomsBatch
    // This could be stored in a static and accessed from anywhere if required
    // It is the lifetime of this object that controls the lifetime of all of the elements that are added
    let thread_pool = Arc::new(ThreadPool::<RandomsBatch>::new(2));

    // Create a thread pool batcher, provides a mechanism for communicating with the thread pool.
    let thread_pool_batcher =
        ThreadPoolBatcherConcrete::<RandomsBatch>::new(Arc::downgrade(&thread_pool));

    // as a shared thread pool will be used for all Randoms it is important that the RandomsBatches share an id provider
    // (the Randoms ids need to be unique across all RandomBatches )
    // this id provider uses a mutex to ensure it provides unique ids
    let id_provider = SizedIdProvider::new(IdProviderMutex::new(0));

    // Create 10 requests to create randoms batches
    // Each RandomsBatch will in turn create 100 Randoms.
    // The thread pool for the Randoms will contain 4 dedicated threads
    // each one will in turn contain 10 randoms that will be distributed across a thread pool with 4 threads
    for i in 0..10 {
        thread_pool_batcher.batch_for_send(randoms_batch_init_request::RandomsBatchInitRequest {
            id: i,
            number_of_contained_randoms: 100,
            thread_pool_size: 4,
            id_provider: id_provider.clone(),
        });
    }
    // this call distributes the work across the thread pool and blocks until all of the work is done
    let _: Vec<randoms_batch_init_response::RandomsBatchInitResponse> =
        thread_pool_batcher.send_batch();

    // send 10 requests for the sum of sums
    for i in 0..10 {
        thread_pool_batcher.batch_for_send(sum_of_sums_request::SumOfSumsRequest { id: i % 10 });
    }
    // this call distributes the work across the thread pool and blocks until all of the work is done
    let means: Vec<sum_of_sums_response::SumOfSumsResponse> = thread_pool_batcher.send_batch();

    dbg!(means);
}
