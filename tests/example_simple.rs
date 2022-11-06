use messaging_thread_pool::{
    samples::randoms::{
        randoms_request::{
            init_request::InitRequest, mean_request::MeanRequest, sum_request::SumRequest,
        },
        randoms_response::{
            init_response::InitResponse, mean_response::MeanResponse, sum_response::SumResponse,
            RandomsResponse,
        },
        Randoms,
    },
    thread_pool_batcher::{basic_thread_pool_batcher::BasicThreadPoolBatcher, ThreadPoolBatcher},
    thread_request::ThreadRequest,
    thread_response::ThreadResponse,
};

#[test]
pub fn example_simple_one_level_thread_pool() {
    // creates a thread pool with 4 threads and a mechanism by which to communicate with the threads in the pool.
    // The lifetime of the elements created (the Randoms) will be tied to the life of this struct
    let thread_pool_batcher = BasicThreadPoolBatcher::<Randoms>::new(4);

    // create a 1000 requests to create 'Randoms'
    for i in 0..1000 {
        thread_pool_batcher.batch_for_send(InitRequest { id: i });
    }
    // Send the request to create the 1000 Randoms. Each Randoms will be stored on the
    // thread where it is created
    // They will be assigned to one of the 4 threads based on their ids; [thread = id % 4]
    // This call will block until all 1000 Randoms have been created; the work will be spread across all 4 threads
    let _: Vec<InitResponse> = thread_pool_batcher.send_batch();

    // now create 1000 messages asking them for the sum of their contained random numbers
    for i in 0..1000 {
        thread_pool_batcher.batch_for_send(SumRequest { id: i });
    }
    // Send the messages
    // The message will be routed to the thread to where the targeted element resides
    // Again this call blocks until all of the work is done
    let _sums: Vec<SumResponse> = thread_pool_batcher.send_batch();

    // get the mean of the randoms for element with id 0, this will execute on thread 0
    // this call will block until complete
    let mean0 = thread_pool_batcher
        .batch_for_send(MeanRequest { id: 0 })
        .send_batch::<MeanResponse>()[0]
        .mean;
    println!("{}", mean0);

    // remove element with id 1
    // it wil be dropped from the thread where it was residing
    let response = thread_pool_batcher
        .batch_for_send(ThreadRequest::RemoveElement(1))
        .send_batch::<ThreadResponse<RandomsResponse>>();
    println!("{:?}", response);

    // add a new element with id 1000
    let response = thread_pool_batcher
        .batch_for_send(InitRequest { id: 1000 })
        .send_batch::<ThreadResponse<RandomsResponse>>();
    println!("{:?}", response);

    // all elements are dropped when the basic thread pool batcher is dropped
    // the threads are shutdown and joined back the the main thread
    drop(thread_pool_batcher);
}
