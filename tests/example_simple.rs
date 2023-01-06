// use messaging_thread_pool::{samples::*, thread_request_response::*, ThreadPool};
// use std::iter;

// #[test]
// pub fn example_simple_one_level_thread_pool() {
//     // creates a thread pool with 4 threads and a mechanism by which to communicate with the threads in the pool.
//     // The lifetime of the elements created (the Randoms) will be tied to the life of this struct
//     let thread_pool = ThreadPool::<Randoms>::new(10);

//     // create a 1000 Randoms across the thread pool by sending a thousand add requests.
//     // The creation of these objects (with the keys 0..1000) will be distributed across the 10 threads
//     // in the pool.
//     // Their owning thread will create and store them.
//     // They will not be dropped until they are either requested to be dropped or until the thread pool itself
//     // is dropped.
//     thread_pool
//         .send_and_receive((0..1000usize).map(|i| RandomsAddRequest(i)))
//         .for_each(|response: AddResponse| assert!(response.success()));

//     // now create 1000 messages asking them for the sum of the Randoms objects contained random numbers
//     // The message will be routed to the thread to where the targeted object resides
//     // This call will block until all of the work is done and the responses returned
//     let sums: Vec<SumResponse> = thread_pool
//         .send_and_receive((0..1000usize).map(|i| SumRequest(i)))
//         .collect();
//     assert_eq!(1000, sums.len());

//     // get the mean of the randoms for pool item with id 0, this will execute on thread 0
//     // this call will block until complete
//     let mean_response_0: MeanResponse = thread_pool
//         .send_and_receive(iter::once(MeanRequest(0)))
//         .nth(0)
//         .unwrap();
//     println!("{}", mean_response_0.mean());

//     // remove element with id 1
//     // it will be dropped from the thread where it was residing
//     thread_pool
//         .send_and_receive(iter::once(RemovePoolItemRequest(1)))
//         .for_each(|response: RemovePoolItemResponse| assert!(response.success()));

//     // add a new pool item with id 1000
//     thread_pool
//         .send_and_receive(iter::once(RandomsAddRequest(1000)))
//         .for_each(|response: AddResponse| assert!(response.success()));

//     // all pool items are dropped when the basic thread pool batcher is dropped
//     // the threads are shutdown and joined back the the main thread
//     drop(thread_pool);
// }
