# messaging_thread_pool


[![Crates.io](https://img.shields.io/crates/v/once_cell.svg)](https://crates.io/crates/messaging_thread_pool)
[![API reference](https://docs.rs/messaging_thread_pool/badge.svg)](https://docs.rs/messaging_thread_pool/)

# Overview

`messaging_thread_pool` provides a set traits and structs that allows the construction of a simple typed thread pool.

Instances of the type are distributed across the threads of the thread pool and are tied to their allocated thread for their entire lifetime.\
The library infrastructure then allows the routing of messages to specific instances based on a key.\
Any work required to respond to a message is executed on that instances assigned thread pool thread.\
Response messages are then routed back to the caller via the infrastructure.

It provides simple call schematics, easy to reason about lifetimes and predictable pool behaviour.


The type needs to define an enum of message types and provide implementations of a few simple traits to enable it to be
hosted within the thread pool.

So, for example, a simple type holding a collection of random numbers such as this

```rust
// define what a pool item looks like
pub struct Randoms {
    // pool items require an id so they can be identified within
    // the thread pool
    pub id: u64,  
    pub numbers: Vec<u64>,
}
```

Can be hosted in a thread pool and communicated with via a defined set of messages by providing implementations 
for the `PoolItem` trait.\
This approximately equates to providing a constructor for the pool items, a set of messages and a message processor 

```rust
// defining the api with which to communicate with the pool item
pub enum RandomsApi {
    Mean(RequestResponse<MEAN, MeanRequest, MeanResponse>),
    Sum(RequestResponse<SUM, SumRequest, SumResponse>),
}

// a request needs to contain the id of the targeted pool item
pub struct MeanRequest(pub usize);

// a response contains the results of the operation
pub struct MeanResponse {
    pub id: usize,
    pub mean: u128,
}

// requests and responses are associated at compile time by a 
// common constant
pub const SUM: usize = 1;

// this function (from the PoolItem trait) defines what to do 
// on receipt of a request and how to respond to it
fn process_message(&mut self, request: &Self::Api) 
        -> ThreadRequestResponse<Self> {
    match request {
        // calculate the mean of the contained randoms and 
        // return the result
        RandomsApi::Mean(request) => MeanResponse {
            id: request.id(),
            mean: self.mean(),
        }
        .into(),
        // calculate the sum of the contained randoms and return
        RandomsApi::Sum(request) => SumResponse {
            id: request.id(),
            sum: self.sum(),
        }
        .into(),
    }
}

// a request is defined defined to construct a pool item
pub struct RandomsAddRequest(pub usize);

// ... and the implementation of this function (in the
// PoolItem trait) defines how to use that message to
// construct a new pool item
fn new_pool_item(request: &Self::Init) 
        -> Result<Self, NewPoolItemError> {
    Ok(Randoms::new(request.0))
}

```

With this infrastructure in place a pool item can then use the library provided structs 
to host instances of the pool items in a fixed sized thread pool. 


```rust
use std::iter;
use messaging_thread_pool::{samples::*,
     thread_request_response::*,
     ThreadPool};

    // creates a thread pool with 4 threads and a mechanism 
    // by which to communicate with the threads in the pool.
    // The lifetime of the structs created (the Randoms) 
    // will be tied to the life of this struct
    let thread_pool = ThreadPool::<Randoms>::new(10);

    // create a 1000 Randoms across the thread pool by 
    // sending a thousand add requests.
    // The creation of these objects (with the keys 0..1000)
    // will be distributed across the 10 threads in the pool.
    // Their owning thread will create and store them.
    // They will not be dropped until they are either 
    // requested to be dropped or until the thread pool
    // itself is dropped.
    thread_pool
        .send_and_receive((0..1000usize)
        .map(|i| RandomsAddRequest(i)))
        .for_each(|response: AddResponse| 
            assert!(response.success()));

    // now create 1000 messages asking each of them for the sum of
    // the Randoms objects contained random numbers
    // The message will be routed to the thread to where
    // the targeted object resides
    // This call will block until all of the work is done and
    // the responses returned
    let sums: Vec<SumResponse> = thread_pool
        .send_and_receive((0..1000usize)
        .map(|i| SumRequest(i)))
        .collect();
    assert_eq!(1000, sums.len());

    // now get the mean of the randoms for object with id 0, this 
    // will execute on thread 0.
    // this call will block until complete
    let mean_response_0: MeanResponse = thread_pool
        .send_and_receive(iter::once(MeanRequest(0)))
        .nth(0)
        .unwrap();
    println!("{}", mean_response_0.mean());

    // remove object with id 1
    // it will be dropped from the thread where it was residing
    // freeing up any memory it was using
    thread_pool
        .send_and_receive(iter::once(RemovePoolItemRequest(1)))
        .for_each(|response: RemovePoolItemResponse| 
            assert!(response.success()));

    // add a new object with id 1000
    thread_pool
        .send_and_receive(iter::once(RandomsAddRequest(1000)))
        .for_each(|response: AddResponse| 
            assert!(response.success()));

    // all objects are dropped when the thread pool is
    // dropped, the worker threads are shutdown and
    // joined back the the main thread
    drop(thread_pool);

```

The original motivation for the library was to cope with hierarchies of long-lived dependent objects, each of which
were required to have their own thread pools to avoid any complex threading dependencies.
All of the operations were CPU bound.

It is important to note that unless the operations being performed are quite long running (>50ms) then the costs of
messaging infrastructure starts to become significant and will start to eat into the benefits of having multiple threads


