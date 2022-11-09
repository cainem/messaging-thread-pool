# messaging_thread_pool


[![Crates.io](https://img.shields.io/crates/v/once_cell.svg)](https://crates.io/crates/messaging_thread_pool)
[![API reference](https://docs.rs/messaging_thread_pool/badge.svg)](https://docs.rs/messaging_thread_pool/)

# Overview

`messaging_thread_pool` provides a set traits and structs that allows the construction of a simple typed thread pool.

The type needs to define an enum of message types and provide implementations of a few simple traits to enable it to be
hosted within the thread pool.

So, for example, a simple type such holding a collection of random numbers such as this

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Randoms {
    pub id: u64,  // elements require an id so they can be identified within the thread pool
    pub numbers: Vec<u64>,
}
```

Can be hosted in a thread pool and communicated with via a defined set of messages by providing implementations for the `Element` trait. 
This approximately equates to providing a set of messages, a message processor and a function for creating new elements

```rust

// defining the recognised requests
pub enum RandomsRequest {
    Mean(MeanRequest),
    Sum(SumRequest),
    Init(RandomsInitRequest),
}

// defining what to do on receipt of the messages and how to response
fn process_message(&mut self, request: &RandomsRequest) -> RandomsResponse {
    match request {
        RandomsRequest::Mean(get_mean_request) => MeanResponse {
            id: get_mean_request.id,
            mean: self.mean(),
        }
        .into(),
        RandomsRequest::Sum(_get_state) => SumResponse {
            id: self.id,
            sum: self.sum(),
        }
        .into(),
        :
        :
    }
}

// how to create a element in the thread pool
fn new_element(request: &RandomsRequest) -> (Option<Self>, RandomsResponse) {
    match request {
        RandomsRequest::Init(init) => (
            Some(Randoms::new(init.id)),
            RandomsResponse::Init(RandomsInitResponse { id: init.id }),
        ),
        _ => panic!("expected init only"),
    }
}

```

Once this is done the element can then use the library provided structs to host instances of the element in a fixed sized thread pool. 

This provides simple call schematics, easy to reason about lifetimes and predictable pool behaviour.

```rust

// creates a thread pool with 4 threads and a mechanism by which to communicate with the threads in the pool.
// The lifetime of the elements created (the Randoms) will be tied to the life of this struct
let thread_pool_batcher = BasicThreadPoolBatcher::<Randoms>::new(4);

// create a 1000 requests to create 'Randoms'
for i in 0..1000 {
    thread_pool_batcher.batch_for_send(randoms_init_request::RandomsInitRequest { id: i });
}
// Send the request to create the 1000 Randoms. Each Randoms will be stored on the
// thread where it is created
// They will be assigned to one of the 4 threads based on their ids; [thread = id % 4]
// This call will block until all 1000 Randoms have been created; the work will be spread across all 4 threads
let _: Vec<randoms_init_response::RandomsInitResponse> = thread_pool_batcher.send_batch();

// now create 1000 messages asking them for the sum of their contained random numbers
for i in 0..1000 {
    thread_pool_batcher.batch_for_send(sum_request::SumRequest { id: i });
}
// Send the messages
// The message will be routed to the thread to where the targeted element resides
// Again this call blocks until all of the work is done
let sums: Vec<sum_response::SumResponse> = thread_pool_batcher.send_batch();
assert_eq!(1000, sums.len());

// get the mean of the randoms for element with id 0, this will execute on thread 0
// this call will block until complete
let mean0 = thread_pool_batcher
    .batch_for_send(mean_request::MeanRequest { id: 0 })
    .send_batch::<mean_response::MeanResponse>()[0]
    .mean;
println!("{}", mean0);

// remove element with id 1
// it wil be dropped from the thread where it was residing
let responses = thread_pool_batcher
    .batch_for_send(ThreadRequest::RemoveElement(1))
    .send_batch::<ThreadResponse<RandomsResponse>>();
println!("{:?}", responses);

// add a new element with id 1000
let responses = thread_pool_batcher
    .batch_for_send(randoms_init_request::RandomsInitRequest { id: 1000 })
    .send_batch::<ThreadResponse<RandomsResponse>>();
println!("{:?}", responses);

// all elements are dropped when the basic thread pool batcher is dropped
// the threads are shutdown and joined back the the main thread
drop(thread_pool_batcher);

```

The original motivation for the library was to cope with hierarchies of long-lived dependent objects, each of which were required to have their own thread pools to avoid any complex threading dependencies.
All of the operations were CPU bound.

It is important to note that unless the operations being performed are quite long running (>50ms) then the costs of messaging infrastructure starts to become significant and will start to eat into the benefits of having multiple threads


