use std::iter;

use messaging_thread_pool::{samples::*, thread_request_response::*, ThreadPool};

#[test]
pub fn example_simple_one_level_thread_pool() {
    // creates a thread pool with 4 threads and a mechanism by which to communicate with the threads in the pool.
    // The lifetime of the elements created (the Randoms) will be tied to the life of this struct
    let thread_pool = ThreadPool::<Randoms>::new(1);

    thread_pool
        .send_and_receive((0..10000usize).map(|i| RandomsAddRequest(i)))
        .for_each(|response: AddResponse| assert!(response.success()));

    // now create 1000 messages asking them for the sum of their contained random numbers
    // Send the messages
    // The message will be routed to the thread to where the targeted element resides
    // Again this call blocks until all of the work is done
    let sums: Vec<SumResponse> = thread_pool
        .send_and_receive((0..10000usize).map(|i| SumRequest(i)))
        .collect();
    assert_eq!(1000, sums.len());

    // get the mean of the randoms for element with id 0, this will execute on thread 0
    // this call will block until complete
    let mean_response_0: MeanResponse = thread_pool
        .send_and_receive(iter::once(MeanRequest(0)))
        .nth(0)
        .unwrap();
    println!("{}", mean_response_0.mean());

    // remove element with id 1
    // it will be dropped from the thread where it was residing
    thread_pool
        .send_and_receive(iter::once(RemovePoolItemRequest(1)))
        .for_each(|response: RemovePoolItemResponse| assert!(response.success()));

    // add a new element with id 1000
    thread_pool
        .send_and_receive(iter::once(RandomsAddRequest(1000)))
        .for_each(|response: AddResponse| assert!(response.success()));

    // all elements are dropped when the basic thread pool batcher is dropped
    // the threads are shutdown and joined back the the main thread
    drop(thread_pool);
}
