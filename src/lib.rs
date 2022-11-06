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
    use crate::{samples::randoms::Randoms, ThreadPool};

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
