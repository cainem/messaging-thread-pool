use std::sync::Arc;

use messaging_thread_pool::{
    global_test_scope::global_test_scope, id_provider::id_provider_mutex::IdProviderMutex,
    samples::*, *,
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
    // It is the lifetime of this struct that controls the lifetime of all the pool items that are added
    let randoms_batch_thread_pool = ThreadPool::<RandomsBatch<ThreadPool<Randoms>>>::new(1);

    // Create another thread pool to be used by the children of the RandomsBatches (which are Randoms)
    // The arrangement here is to have this thread shared by all the Randoms regardless of which RandomsBatch
    // is their parent. For this reason this thread pool is wrapped in an Arc.
    let randoms_thread_pool = Arc::new(ThreadPool::<Randoms>::new(2));

    // as a shared thread pool will be used for all Randoms it is important that the RandomsBatches share an id provider
    // (the Randoms ids need to be unique across all RandomBatches )
    // this id provider uses a mutex to ensure it provides unique ids
    let id_provider = Arc::new(IdProviderMutex::new(0));

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
        .expect("thread pool to be available")
        .for_each(|response: AddResponse| assert!(response.result().is_ok()));

    // now request the "sum of sums" from each RandomBatch by sending a request to each of the RandomsBatches
    // This generates a large amount of work across the 2 thread pools.
    // One thread pool is dedicated to the work of running the RandomsBatches, the other is dedicated to the
    // work of running the Randoms

    // this call distributes the work across the thread pool and blocks until all of the work is done
    let sum_of_sums: Vec<u128> = randoms_batch_thread_pool
        .send_and_receive((0..10).map(SumOfSumsRequest))
        .expect("thread pool to be available")
        .map(|response: SumOfSumsResponse| response.sum_of_sums())
        .collect();

    dbg!(sum_of_sums);
}

#[test]
pub fn example_random_batches_with_mock_thread_pool() {
    // the id provider will not be used in this example but is required to construct a RandomsBatch
    let id_provider = Arc::new(IdProviderMutex::new(0));

    // Create a mock thread pool that will be called from inside of the RandomsBatch when sum-of_sums is called.
    // Constructed like this is states that is expecting to receive 2 SumRequests requests (it will verify this)
    // and in return it will return 2 SumResponses
    // This enables the functionality of the sum_of_sums function to be tests without constructing a real thread pool
    let randoms_thread_pool =
        SenderAndReceiverMock::<Randoms, SumRequest>::new_with_expected_requests(
            vec![SumRequest(2), SumRequest(4)],
            vec![SumResponse { id: 2, sum: 2 }, SumResponse { id: 4, sum: 4 }],
        );

    // new create a RandomsBatch using the mock thread pool
    let target = RandomsBatch {
        id: 1,
        contained_random_ids: vec![2, 4],
        id_provider,
        randoms_thread_pool: Arc::new(randoms_thread_pool),
    };

    // now call sum_of_sums
    // this will fire the 2 expected requests and receive back the hard coded responses
    let result = target.sum_of_sums();

    // the sum of sums will be 6 (2 + 4)
    assert_eq!(6, result);
}
