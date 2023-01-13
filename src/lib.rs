//! # Messaging thread pool
//!
//! Messaging thread pool is a collection of traits and structs for setting up a simple fix sized thread pool
//! which holds a collection of a given type.
//!
//! Instances of the objects are identified by an id which is unique within the thread pool.
//!
//! Objects are distributed across the thread pool based on their id and ownership of the
//! object is held there
//!
//! Objects are communicated with via a user defined set of messages which effectively form an api.
//! These messages are sent and received over crossbeam channels.
//!
//! The object need to implement a set of simple traits and define a set of request/response messages
//! to allow the thread pool infrastructure to handle the objects and to route messages to them.
//!
//! The lifetimes of the objects are easy to reason about as is the behaviour of the thread pools themselves.
//!
//! The original motivation was to provide support for a hierarchy of dependent, long-lived objects,
//! that each required their own thread pools to avoid complex threading dependencies
//! The objects in the thread pools were all CPU bound i.e. did not perform any significant I/O
//!
//! # Example
//! ```
//! use std::iter;
//! use messaging_thread_pool::{*, samples::*};
//!
//!    // creates a thread pool with 4 threads.
//!    // The lifetime of the elements created (the Randoms in this case) will be tied to the
//!    // life of this struct
//!    let thread_pool = ThreadPool::<Randoms>::new(10);
//!
//!    // create a 1000 Randoms across the thread pool by sending a thousand add requests.
//!    // The creation of these objects (with the keys 0..1000) will be distributed across
//!    // the 10 threads in the pool.
//!    // Their owning thread will create and store them.
//!    // They will not be dropped until they are either requested to be dropped or until the
//!    // thread pool itself is dropped.
//!    thread_pool
//!        .send_and_receive((0..1000usize).map(|i| RandomsAddRequest(i)))
//!        .expect("thread pool to be available")
//!        .for_each(|response: AddResponse| assert!(response.success()));
//!
//!    // now create 1000 messages asking them for the sum of the Randoms objects contained
//!    // random numbers.
//!    // The message will be routed to the thread to where the targeted object resides
//!    // This call will block until all of the work is done and the responses returned
//!    let sums: Vec<SumResponse> = thread_pool
//!        .send_and_receive((0..1000usize).map(|i| SumRequest(i)))
//!        .expect("thread pool to be available")
//!        .collect();
//!    assert_eq!(1000, sums.len());
//!
//!    // get the mean of the randoms for object with id 0, this will execute on thread 0
//!    // this call will block until complete
//!    let mean_response_0: MeanResponse = thread_pool
//!        .send_and_receive(iter::once(MeanRequest(0)))
//!        .expect("thread pool to be available")
//!        .nth(0)
//!        .unwrap();
//!    println!("{}", mean_response_0.mean());
//!
//!    // remove object with id 1
//!    // it will be dropped from the thread where it was residing
//!    thread_pool
//!        .send_and_receive(iter::once(RemovePoolItemRequest(1)))
//!        .expect("thread pool to be available")
//!        .for_each(|response: RemovePoolItemResponse| assert!(response.success()));
//!
//!    // add a new object with id 1000
//!    thread_pool
//!        .send_and_receive(iter::once(RandomsAddRequest(1000)))
//!        .expect("thread pool to be available")
//!        .for_each(|response: AddResponse| assert!(response.success()));
//!
//!    // all objects are dropped when the basic thread pool batcher is dropped
//!    // the threads are shutdown and joined back the the main thread
//!    drop(thread_pool);
//! ```
//!
//! # Limitations
//!
//! The thread pool cannot be dynamically sized.\
//! It is fixed at creation.\
//! As there is a ThreadShutdown request it could be implied that therefore there should be a ThreadCreation request.
//! This is not the case and it is not intended that individual threads will be shutdown in isolation and in fact
//! this will lead to the thread pool panicking.\
//! The shutdown request is intended to be called only when the whole thread pool is finished with and in fact it
//! is probably best to avoid using it and to just drop the thread pool (which internally sends out all the required shutdown messages).\
//!
//! It was not really intended for anything other than long-lived CPU bound elements.
//!
use std::sync::RwLock;

use thread_endpoint::ThreadEndpoint;

pub mod global_test_scope;
pub mod id_provider;
pub mod samples;
pub mod sender_couplet;

mod drop;
mod id_targeted;
mod new;
mod pool_item;
mod pool_thread;
mod pool_thread_old;
mod receive;
mod request_response;
mod request_with_response;
mod send;
mod send_and_receive;
mod sender_and_receiver;
mod shutdown;
mod thread_endpoint;
mod thread_request_response;

pub use id_targeted::IdTargeted;
pub use pool_item::*;
pub use request_response::RequestResponse;
pub use request_with_response::RequestWithResponse;
pub use sender_and_receiver::*;
pub use sender_couplet::*;
pub use thread_request_response::*;

/// This struct represents a pool of threads that can target a particular type of
/// resource (a resource being a struct that implements [`PoolItem`])
///
/// In order to allow for distribution over multiple threads each resource must have an id
/// that allows for routing to a particular thread.
#[derive(Debug)]
pub struct ThreadPool<P>
where
    P: PoolItem,
{
    thread_endpoints: RwLock<Vec<ThreadEndpoint<P>>>,
}

impl<P> ThreadPool<P>
where
    P: PoolItem,
{
    /// This function returns the number of threads in the thread pool
    pub fn thread_count(&self) -> usize {
        self.thread_endpoints
            .read()
            .expect("read should never be poisoned")
            .len()
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
