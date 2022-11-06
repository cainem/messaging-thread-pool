use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use messaging_thread_pool::{
    samples::randoms::{
        randoms_request::init_request::InitRequest, randoms_response::init_response::InitResponse,
        Randoms,
    },
    thread_pool_batcher::thread_pool_batcher_concrete::ThreadPoolBatcherConcrete,
    ThreadPool,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create 1000 randoms", |b| {
        b.iter(|| {
            let thread_pool = Arc::new(ThreadPool::<Randoms>::new(20));
            let thread_pool_batcher =
                ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));
            for i in 0..1000 {
                thread_pool_batcher.batch_for_send(InitRequest { id: i });
            }
            let _: Vec<InitResponse> = thread_pool_batcher.send_batch();
            thread_pool.shutdown();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
