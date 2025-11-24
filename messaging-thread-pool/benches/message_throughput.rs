use criterion::{Criterion, criterion_group, criterion_main};
use messaging_thread_pool::{ThreadPool, samples::*};

pub fn criterion_benchmark(c: &mut Criterion) {
    let thread_pool = ThreadPool::<Randoms>::new(10);

    // create 100 randoms
    thread_pool
        .send_and_receive((0..100).map(RandomsAddRequest))
        .expect("thread pool to exist")
        .for_each(|_| {});

    c.bench_function("sum 100 randoms", |b| {
        b.iter(|| {
            // send sum requests to all 100 randoms
            thread_pool
                .send_and_receive((0..100).map(SumRequest))
                .expect("thread pool to exist")
                .for_each(|_| {});
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
