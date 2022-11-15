use std::{sync::RwLock, thread::spawn};

use crossbeam_channel::unbounded;
use tracing::{event, Level};

use crate::{
    pool_item::PoolItem, pool_thread::PoolThread, sender_couplet_2::SenderCouplet2,
    thread_endpoint::ThreadEndpoint, ThreadPool,
};

impl<E> ThreadPool<E>
where
    // 'static - the Element cannot contain any references as it isn't guaranteed to live long enough
    // due to it being passed to another thread
    E: PoolItem + 'static,
{
    /// This function creates a new [`ThreadPool`]
    ///
    /// Internally it creates a collection of threads.
    /// It has the ability to communicate with the threads via a vec of channels
    /// (there is one channel for each spawned thread)
    ///
    /// The number of threads is determined by the passed in thread_pool_size
    pub fn new(thread_pool_size: usize) -> Self {
        assert!(
            thread_pool_size > 0,
            "thread pool must have at least one thread"
        );

        let mut building = Vec::<ThreadEndpoint<E>>::new();

        for i in 0..thread_pool_size as u64 {
            let (send_to_thread, receive_from_pool) = unbounded::<SenderCouplet2<E>>();

            let join_handle = spawn(move || {
                // set default tracing subscribers for thread
                let tracing_guards = E::add_pool_thread_tracing(i);

                // start a new thread with id i
                let mut pool_thread = PoolThread::<E>::new(i, receive_from_pool);

                event!(Level::INFO, "starting message loop");

                // enter the "infinite" message loop where messages will be received
                pool_thread.message_loop();

                // drop the threads tracing subscriber
                drop(tracing_guards);

                // return the pool thread id in the join handle
                i
            });

            building.push(ThreadEndpoint {
                sender: send_to_thread,
                join_handle,
            });
        }

        ThreadPool {
            thread_endpoints: RwLock::new(building),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        samples::*, thread_request_response::thread_shutdown_response::ThreadShutdownResponse,
        ThreadPool,
    };

    #[test]
    fn new_called_with_thread_pool_size_2_two_threads_created() {
        let result = ThreadPool::<Randoms>::new(2);

        // one thread created
        assert_eq!(2, result.thread_endpoints.read().unwrap().len());

        // thread had id 1
        assert_eq!(
            result.shutdown(),
            &[
                ThreadShutdownResponse::new(0, vec![]),
                ThreadShutdownResponse::new(1, vec![])
            ]
        );
    }

    #[test]
    fn new_called_with_thread_pool_size_1_one_thread_created() {
        let result = ThreadPool::<Randoms>::new(1);

        // one thread created
        assert_eq!(1, result.thread_endpoints.read().unwrap().len());

        // thread had id 0
        assert_eq!(result.shutdown(), &[ThreadShutdownResponse::new(0, vec![])]);
    }
}
