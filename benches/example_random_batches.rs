use criterion::{criterion_group, criterion_main, Criterion};
use messaging_thread_pool::id_provider::id_provider_mutex::IdProviderMutex;
use messaging_thread_pool::samples::{
    Randoms, RandomsBatch, RandomsBatchAddRequest, SumOfSumsRequest, SumOfSumsResponse,
};
use messaging_thread_pool::{AddResponse, ThreadPool};
use std::sync::Arc;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.sample_size(10);

    group.bench_function("run multiple level example", |b| {
        b.iter(|| {
            //global_test_scope(LevelFilter::OFF);

            // Create a thread pool for RandomsBatch
            // It is the lifetime of this struct that controls the lifetime of all the pool items that are added
            let randoms_batch_thread_pool = ThreadPool::<RandomsBatch<ThreadPool<Randoms>>>::new(4);

            // Create another thread pool to be used by the children of the RandomsBatches (which are Randoms)
            // The arrangement here is to have this thread shared by all the Randoms regardless of which RandomsBatch
            // is their parent. For this reason this thread pool is wrapped in an Arc.
            let randoms_thread_pool = Arc::new(ThreadPool::<Randoms>::new(10));

            // as a shared thread pool will be used for all Randoms it is important that the RandomsBatches share an id provider
            // (the Randoms ids need to be unique across all RandomBatches )
            // this id provider uses a mutex to ensure it provides unique ids
            let id_provider = Arc::new(IdProviderMutex::new(0));

            // Create 10 requests to create randoms batches
            // Each RandomsBatch will in turn create 10_000 Randoms.
            // The thread pool for the Randoms will contain 4 dedicated threads
            // each one will in turn contain 10 randoms that will be distributed across a thread pool with 4 threads

            // this call distributes the work across the thread pool and blocks until all the work is done
            randoms_batch_thread_pool
                .send_and_receive((0..10).map(|id| RandomsBatchAddRequest {
                    id,
                    number_of_contained_randoms: 10_000,
                    id_provider: id_provider.clone(),
                    randoms_thread_pool: Arc::clone(&randoms_thread_pool),
                }))
                .expect("thread pool to be available")
                .for_each(|response: AddResponse| assert!(response.result().is_ok()));

            // now request the "sum of sums" from each RandomBatch by sending a request to each of the RandomsBatches
            // This generates a large amount of work across the 2 thread pools.
            // One thread pool is dedicated to the work of running the RandomsBatches, the other is dedicated to the
            // work of running the Randoms

            // this call distributes the work across the thread pool and blocks until all the work is done
            let _sum_of_sums: Vec<u128> = randoms_batch_thread_pool
                .send_and_receive((0..10).map(SumOfSumsRequest))
                .expect("thread pool to be available")
                .map(|response: SumOfSumsResponse| response.sum_of_sums())
                .collect();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
