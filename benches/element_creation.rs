// use criterion::{criterion_group, criterion_main, Criterion};
// use messaging_thread_pool::{samples::*, thread_request_response::*, ThreadPool};

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("create 1000 randoms", |b| {
//         b.iter(|| {
//             let thread_pool = ThreadPool::<Randoms>::new(20);

//             thread_pool
//                 .send_and_receive((0..1000).map(|i| RandomsAddRequest(i)))
//                 .for_each(|_: AddResponse| {});

//             thread_pool.shutdown();
//         })
//     });
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
