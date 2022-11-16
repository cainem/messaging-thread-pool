//! # Messaging thread pool
//!
//! Messaging thread pool is a collection of traits and structs for setting up a simple fix sized thread pool
//! which holds a collection of objects all of the same type.
//!
//! Instances of the objects are identified by an id which is unique within the thread pool.
//!
//! Objects are distributed across the pool based on their id and they always live on the thread
//! on which they were created and any state they have is stored there.
//!
//! They are communicated with via a defined set of messages which effectively form an api.
//! These messages are sent and received over crossbeam channels.
//!
//! The object need to implement a set of simple traits and define a set of request/response messages
//! to allow the thread pool infrastructure to handle the objects and to route messages to them.
//!
//! The lifetimes of the objects are easy to reason about as is the behaviour of the thread pools themselves.
//!
//! The original motivation was to provide support for a hierarchy of dependent objects that each required their
//! own thread pools to avoid complex threading dependencies
//! The objects in the thread pools were all CPU bound and so there was no need for any async/await support.
//!
//! # Example
//! ```
//! use messaging_thread_pool::{
//!    samples::*,
//!    thread_pool_batcher::{BasicThreadPoolBatcher, ThreadPoolBatcher},
//!    thread_request::ThreadRequest,
//!    thread_response::ThreadResponse,
//! };
//!    // creates a thread pool with 4 threads and a mechanism by which to communicate with
//!    // the threads in the pool.
//!    // The lifetime of the elements created (the [Randoms]) will be tied to the life of this struct
//!    let thread_pool_batcher = BasicThreadPoolBatcher::<Randoms>::new(4);
//!
//!    // create a 1000 requests to create 'Randoms'
//!    for i in 0..1000 {
//!        thread_pool_batcher.batch_for_send(randoms_init_request::RandomsInitRequest { id: i });
//!    }
//!
//!    // Send the request to create the 1000 Randoms. Each Randoms will be stored on the
//!    // thread where it is created
//!    // They will be assigned to one of the 4 threads based on their ids; [thread = id % 4]
//!    // This call will block until all 1000 Randoms have been created; the work will be
//!    // spread across all 4 threads
//!    let _: Vec<randoms_init_response::RandomsInitResponse> = thread_pool_batcher.send_batch();
//!
//!    // now create 1000 messages asking them for the sum of their contained random numbers
//!    for i in 0..1000 {
//!        thread_pool_batcher.batch_for_send(sum_request::SumRequest { id: i });
//!    }
//!    // Send the messages
//!    // The message will be routed to the thread to where the targeted element resides
//!    // Again this call blocks until all of the work is done
//!    let sums: Vec<sum_response::SumResponse> = thread_pool_batcher.send_batch();
//!    assert_eq!(1000, sums.len());
//!
//!    // get the mean of the randoms for element with id 0, this will execute on thread 0
//!    // this call will block until complete
//!    let mean0 = thread_pool_batcher
//!        .batch_for_send(mean_request::MeanRequest { id: 0 })
//!        .send_batch::<mean_response::MeanResponse>()[0]
//!        .mean;
//!    println!("{}", mean0);
//!
//!    // remove element with id 1
//!    // it will be dropped from the thread where it was residing
//!    let responses = thread_pool_batcher
//!        .batch_for_send(ThreadRequest::RemoveElement(1))
//!        .send_batch::<ThreadResponse<RandomsResponse>>();
//!    println!("{:?}", responses);
//!
//!    // add a new element with id 1000
//!    let responses = thread_pool_batcher
//!        .batch_for_send(randoms_init_request::RandomsInitRequest { id: 1000 })
//!        .send_batch::<ThreadResponse<RandomsResponse>>();
//!    println!("{:?}", responses);
//!
//!    // all elements are dropped when the basic thread pool batcher is dropped
//!    // the threads are shutdown and joined back the the main thread
//!    drop(thread_pool_batcher);
//! ```
//!
//! # Panics
//!
//! There are a whole host of reasons currently why the thread pool will panic.\
//! Due to my own selfish requirements if anything goes wrong the thread pool panics.
//!
//! The list of reasons includes the following:-
//!
//! * If a request is made to create an element whose id already exists.
//! * If a request is made to remove an element that doesn't exist.
//! * If a request for a given id does not receive a response with the corresponding id.
//! * If a request is made to shutdown or abort a thread with a given id does not exist.
//!
//! Also if any of the internal elements themselves panic there is no protection provided against this
//! and the thread panic, which in turn causes the thread pool to eventually panic.
//!
//! # Limitations
//!
//! As mentioned previous it was written specifically with one use case in mind.
//! The handling of errors and panics is poor.\
//! There is an underlying assumption that if anything goes wrong then there is no value in continuing.
//!
//! The thread pool cannot be dynamically sized.\
//! It is fixed at creation.\
//! As there is a ThreadShutdown request it could be implied that therefore there should be a ThreadCreation request.
//! This is not the case and it is not intended that individual threads will be shutdown in isolation and in fact
//! this will lead to the thread pool panicking.\
//! The shutdown request is intended to be called only when the whole thread pool is finished with and in fact it
//! is probably best to avoid using it and to just drop the thread pool (which internally sends out all the required shutdown messages).\
//!
//! It was not intended for anything other than long-lived CPU bound elements.
//!
use std::{num::NonZeroUsize, sync::RwLock};

use pool_item::PoolItem;
use thread_endpoint::ThreadEndpoint;

pub mod id_provider;
pub mod id_targeted;
pub mod samples;
pub mod thread_pool_batcher;
pub mod thread_request;
pub mod thread_response;

mod drop;
mod new;
pub mod new2;
pub mod pool_item;
mod pool_thread;
mod pool_thread_2;
mod receive;
pub mod request_response_pair;
mod send;
mod send_and_receive;
pub mod sender_couplet_2;
mod shutdown;
mod thread_endpoint;
pub mod thread_request_response;

/// This struct represents a pool of threads that can target a particular type of
/// resource (a resource being a struct that implements Element)
///
/// In order to allow for distribution over multiple threads each resource must have an id
/// that allows for routing to a particular thread.
///
/// It is necessary when request are made
#[derive(Debug)]
pub struct ThreadPool<E>
where
    E: PoolItem,
{
    thread_endpoints: RwLock<Vec<ThreadEndpoint<E>>>,
}

impl<E> ThreadPool<E>
where
    E: PoolItem,
{
    /// This function returns the number of threads in the thread pool
    pub fn thread_count(&self) -> NonZeroUsize {
        NonZeroUsize::new(
            self.thread_endpoints
                .read()
                .expect("read should never be poisoned")
                .len(),
        )
        .expect("number of threads to be greater than zero")
    }
}

#[cfg(test)]
mod tests {
    use crate::{samples::*, ThreadPool};

    #[test]
    fn thread_pool_size_2_thread_count_2() {
        let result = ThreadPool::<Randoms>::new(2);

        // one thread created
        assert_eq!(2, usize::from(result.thread_count()));

        // shutdown the thread pool
        result.shutdown();
    }

    #[test]
    fn thread_pool_size_1_thread_count_1() {
        let result = ThreadPool::<Randoms>::new(1);

        // one thread created
        assert_eq!(1, usize::from(result.thread_count()));

        // shutdown the thread pool
        result.shutdown();
    }
}
