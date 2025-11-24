//! # Messaging thread pool
//!
//! Messaging thread pool is a collection of traits and structs for setting up a simple fixed-sized thread pool
//! which holds a collection of a given type.
//!
//! Instances of the objects are identified by an id which is unique within the thread pool.
//!
//! Objects are distributed across the thread pool based on their id and ownership of the
//! object is held there.
//!
//! Objects are communicated with via a user defined set of messages which effectively form an API.
//! These messages are sent and received over crossbeam channels.
//!
//! The objects need to implement a set of simple traits and define a set of request/response messages
//! to allow the thread pool infrastructure to handle the objects and to route messages to them.
//!
//! The lifetimes of the objects are easy to reason about, as is the behaviour of the thread pools themselves.
//!
//! The original motivation was to provide support for a hierarchy of dependent, long-lived objects,
//! that each required their own thread pools to avoid complex threading dependencies.
//! The objects in the thread pools were all CPU bound i.e. did not perform any significant I/O.
//!
//! # Example
//! ```
//! use messaging_thread_pool::{ThreadPool, samples::*};
//!
//!    // Create a thread pool with 4 threads
//!    let thread_pool = ThreadPool::<ChatRoom>::new(4);
//!
//!    // Create two chat rooms (ID 1 and 2)
//!    // The pool will route these to the appropriate threads based on ID
//!    thread_pool
//!        .send_and_receive(vec![
//!            ChatRoomInit(1),
//!            ChatRoomInit(2),
//!        ].into_iter())
//!        .expect("creation requests")
//!        .for_each(|_| {});
//!
//!    // Post messages to Room 1
//!    thread_pool
//!        .send_and_receive(vec![
//!            PostRequest(1, "Alice".to_string(), "Hello!".to_string()),
//!            PostRequest(1, "Bob".to_string(), "Hi Alice!".to_string()),
//!        ].into_iter())
//!        .expect("messages to send")
//!        .for_each(|response| {
//!            // The response is the index of the message
//!            assert!(response.result < 100);
//!        });
//!
//!    // Get history from Room 1
//!    let history = thread_pool
//!        .send_and_receive(vec![GetHistoryRequest(1)].into_iter())
//!        .expect("request to send")
//!        .next()
//!        .expect("response")
//!        .result;
//!
//!    assert_eq!(history.len(), 2);
//!    assert_eq!(history[0], "Alice: Hello!");
//! ```
//!
//! # Limitations
//!
//! The thread pool cannot be dynamically sized.\
//! It is fixed at creation.\
//! As there is a ThreadShutdown request it could be implied that therefore there should be a ThreadCreation request.
//! This is not the case, and it is not intended that individual threads will be shutdown in isolation and in fact
//! this will lead to the thread pool panicking.
//! The shutdown request is intended to be called only when the whole thread pool is finished with and in fact it
//! is probably best to avoid using it and to just drop the thread pool (which internally sends out all the required shutdown messages).
//!
//! It was not really intended for anything other than long-lived CPU bound elements.
//!
use std::{cell::RefCell, sync::RwLock};
use thread_endpoint::ThreadEndpoint;

pub mod api_specification;
pub mod global_test_scope;
pub mod id_being_processed;
pub mod id_provider;
pub mod samples;
pub mod sender_couplet;

pub use messaging_thread_pool_macros::pool_item;

mod drop;
mod id_based_blocking;
mod id_targeted;
mod new;
pub mod pool_item;
mod pool_thread;
mod receive;
pub mod request_response;
mod request_with_response;
mod send;
mod send_and_receive;
mod sender_and_receiver;
mod shutdown;
mod thread_endpoint;
pub mod thread_request_response;

pub use id_based_blocking::*;
pub use id_being_processed::*;
pub use id_targeted::IdTargeted;
pub use pool_item::*;
pub use request_response::RequestResponse;
pub use request_with_response::RequestWithResponse;
pub use sender_and_receiver::*;
pub use sender_couplet::*;
pub use thread_request_response::*;

thread_local! {
    pub static ID_BEING_PROCESSED: RefCell<Option<u64>> = const { RefCell::new(None) };
}

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
    use crate::{ThreadPool, samples::*};

    #[test]
    fn thread_pool_size_2_thread_count_2() {
        let result = ThreadPool::<Randoms>::new(2);

        // one thread created
        assert_eq!(2, result.thread_count());

        // shutdown the thread pool
        result.shutdown();
    }

    #[test]
    fn thread_pool_size_1_thread_count_1() {
        let result = ThreadPool::<Randoms>::new(1);

        // one thread created
        assert_eq!(1, result.thread_count());

        // shutdown the thread pool
        result.shutdown();
    }
}
