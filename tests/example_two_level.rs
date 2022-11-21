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
    global_test_scope(LevelFilter::TRACE);

    // Create a thread pool for RandomsBatch
    // It is the lifetime of this object that controls the lifetime of all of the elements that are added
    let thread_pool = ThreadPool::<RandomsBatch>::new(1);

    // as a shared thread pool will be used for all Randoms it is important that the RandomsBatches share an id provider
    // (the Randoms ids need to be unique across all RandomBatches )
    // this id provider uses a mutex to ensure it provides unique ids
    let id_provider = SizedIdProvider::new(IdProviderMutex::new(0));

    // Create 10 requests to create randoms batches
    // Each RandomsBatch will in turn create 100 Randoms.
    // The thread pool for the Randoms will contain 4 dedicated threads
    // each one will in turn contain 10 randoms that will be distributed across a thread pool with 4 threads
    // for i in 0..10 {
    //     thread_pool_batcher.batch_for_send(randoms_batch_init_request::RandomsBatchInitRequest {
    //         id: i,
    //         number_of_contained_randoms: 100,
    //         thread_pool_size: 4,
    //         id_provider: id_provider.clone(),
    //     });
    // }

    // this call distributes the work across the thread pool and blocks until all of the work is done
    thread_pool
        .send_and_receive((0..1).map(|id| RandomsBatchAddRequest {
            id,
            number_of_contained_randoms: 1, //100
            thread_pool_size: 1,            //4
            id_provider: id_provider.clone(),
        }))
        .for_each(|response: AddResponse| assert!(response.success()));

    // send 10 requests for the sum of sums
    // for i in 0..10 {
    //     thread_pool_batcher.batch_for_send(SumOfSumsRequest { id: i % 10 });
    // }

    // this call distributes the work across the thread pool and blocks until all of the work is done
    let sum_of_sums = thread_pool
        .send_and_receive((0..1).map(|id| SumOfSumsRequest(id)))
        .map(|response: SumOfSumsResponse| response.sum_of_sums())
        .nth(0)
        .unwrap();

    dbg!(sum_of_sums);
}
