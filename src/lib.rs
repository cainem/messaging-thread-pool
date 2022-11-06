//! # Messaging thread pool
//!
//! Messaging thread pool is a collection of traits and structs for setting up a simple fix sized thread pool
//! which holds a collection of objects all of the same type.
//!
//! Instances of the objects are identified by an id which is unique within the thread pool.
//! Objects are distributed across the pool based on their id and they always live on the thread
//! on which they were created and any state they have is stored there.
//!
//! They are communicated with via a defined set of messages which effectively form an api.
//! These messages are sent and received over crossbeam channels.
//!
//! The object then implements a set of simple traits to allow the thread pool infrastructure to
//! handle the objects and to route messages to them.
//!
//! The original motivation was to provide a hierarchy of simple, independent thread pools that wouldn't
//! be prone to starving each other.
//! The elements in the thread pools were all CPU bound and so there was no need for any async/await support.
//!
//! Once the donkey work of defining the messages and implementing the necessary traits has been done it does provide
//! a clean interface.
//! The lifetimes of the objects are easy to reason about as is the behaviour of the thread pools themselves
//!
//! # Example
//!    ```
//!    // creates a thread pool with 4 threads and a mechanism by which to communicate with
//!    // the threads in the pool.
//!    // The lifetime of the elements created (the [Randoms]) will be tied to the life of this struct
//!    let thread_pool_batcher = BasicThreadPoolBatcher::<Randoms>::new(4);
//!
//!    // create a 1000 requests to create 'Randoms'
//!    for i in 0..1000 {
//!        thread_pool_batcher.batch_for_send(InitRequest { id: i });
//!    }
//!
//!    // Send the request to create the 1000 Randoms. Each Randoms will be stored on the
//!    // thread where it is created
//!    // They will be assigned to one of the 4 threads based on their ids; [thread = id % 4]
//!    // This call will block until all 1000 Randoms have been created; the work will be
//!    // spread across all 4 threads
//!    let _: Vec<InitResponse> = thread_pool_batcher.send_batch();
//!
//!    // now create 1000 messages asking them for the sum of their contained random numbers
//!    for i in 0..1000 {
//!        thread_pool_batcher.batch_for_send(SumRequest { id: i });
//!    }
//!    // Send the messages
//!    // The message will be routed to the thread to where the targeted element resides
//!    // Again this call blocks until all of the work is done
//!    let sums: Vec<SumResponse> = thread_pool_batcher.send_batch();
//!    assert_eq!(1000, sums.len());
//!
//!    // get the mean of the randoms for element with id 0, this will execute on thread 0
//!    // this call will block until complete
//!    let mean0 = thread_pool_batcher
//!        .batch_for_send(MeanRequest { id: 0 })
//!        .send_batch::<MeanResponse>()[0]
//!        .mean;
//!    println!("{}", mean0);
//!
//!    // remove element with id 1
//!    // it wil be dropped from the thread where it was residing
//!    let responses = thread_pool_batcher
//!        .batch_for_send(ThreadRequest::RemoveElement(1))
//!        .send_batch::<ThreadResponse<RandomsResponse>>();
//!    println!("{:?}", responses);
//!
//!    // add a new element with id 1000
//!    let responses = thread_pool_batcher
//!        .batch_for_send(InitRequest { id: 1000 })
//!        .send_batch::<ThreadResponse<RandomsResponse>>();
//!    println!("{:?}", responses);
//!
//!    // all elements are dropped when the basic thread pool batcher is dropped
//!    // the threads are shutdown and joined back the the main thread
//!    drop(thread_pool_batcher);
//! ```

use std::{num::NonZeroUsize, sync::RwLock};

use element::Element;
use thread_endpoint::ThreadEndpoint;

pub mod element;
pub mod id_provider;
pub mod id_targeted;
pub mod samples;
pub mod thread_pool_batcher;
pub mod thread_request;
pub mod thread_response;
pub mod thread_shutdown_response;

mod drop;
mod new;
mod pool_thread;
mod receive;
mod send;
mod send_and_receive;
mod sender_couplet;
mod shutdown;
mod thread_endpoint;

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
    E: Element,
{
    thread_endpoints: RwLock<Vec<ThreadEndpoint<E>>>,
}

impl<E> ThreadPool<E>
where
    E: Element,
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
