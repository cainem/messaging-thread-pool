use crossbeam_channel::bounded;

use crate::{
    element::Element, thread_request::ThreadRequest, thread_response::ThreadResponse,
    thread_shutdown_response::ThreadShutdownResponse, ThreadPool,
};

impl<E> ThreadPool<E>
where
    E: Element,
{
    /// This function requests that the thread pool shutdowns
    /// It sends the shutdown message to each of it's contained PoolThreads
    /// The sending of this message should cause the message loop to exit and the thread to end
    pub fn shutdown(&self) -> Vec<ThreadShutdownResponse> {
        let (send_to_pool, receive_back_from) = bounded::<ThreadResponse<E::Response>>(0);

        let mut return_codes = Vec::with_capacity(
            self.thread_endpoints
                .read()
                .expect("no poisoned locks")
                .len(),
        );

        for (id, endpoint) in self
            .thread_endpoints
            .write()
            .expect("no poisoned locks")
            .drain(..)
            .enumerate()
        {
            // send straight to each of the thread endpoints
            endpoint.send(
                &send_to_pool,
                ThreadRequest::<E::Request>::ThreadShutdown(id as u64),
            );

            let mut child_threads = Vec::<ThreadShutdownResponse>::default();

            // verify the response back from the message loop; the message loop should now have ended
            match receive_back_from
                .recv()
                .expect("the single response to the shutdown request")
            {
                ThreadResponse::ThreadShutdown(thread_shutdown_payload) => {
                    assert_eq!(
                        thread_shutdown_payload.id(),
                        id as u64,
                        "the passed and returned ids should be the same"
                    );
                    child_threads.append(&mut thread_shutdown_payload.take_children());
                }
                _ => panic!("only a shutdown response expected"),
            }

            // now join the thread
            return_codes.push(ThreadShutdownResponse::new(
                endpoint.join_handle.join().expect("join to succeed"),
                child_threads,
            ));
        }

        return_codes
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        samples::*, thread_pool_batcher::thread_pool_batcher_concrete::ThreadPoolBatcherConcrete,
        thread_shutdown_response::ThreadShutdownResponse, ThreadPool,
    };

    #[test]
    fn two_threads_each_containing_a_sample_element_shutdown_simulates_child_thread_shutdown() {
        let target = Arc::new(ThreadPool::<Randoms>::new(2));

        // two thread created
        assert_eq!(2, target.thread_endpoints.read().unwrap().len());

        let thread_pool_batcher =
            ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&target));

        thread_pool_batcher.batch_for_send(InitRequest { id: 0 });
        thread_pool_batcher.batch_for_send(InitRequest { id: 1 });
        let _: Vec<InitResponse> = thread_pool_batcher.send_batch();

        // thread had id 0, 1
        assert_eq!(
            target.shutdown(),
            &[
                ThreadShutdownResponse::new(0, vec![ThreadShutdownResponse::new(0, vec![])]),
                ThreadShutdownResponse::new(1, vec![ThreadShutdownResponse::new(1, vec![])]),
            ]
        );
    }

    #[test]
    fn two_threads_clean_shutdown_as_expected() {
        let result = ThreadPool::<Randoms>::new(2);

        // two threads created
        assert_eq!(2, result.thread_endpoints.read().unwrap().len());

        // thread had id 0, 1
        assert_eq!(
            result.shutdown(),
            &[
                ThreadShutdownResponse::new(0, vec![]),
                ThreadShutdownResponse::new(1, vec![])
            ]
        );
    }

    #[test]
    fn single_thread_clean_shutdown_as_expected() {
        let result = ThreadPool::<Randoms>::new(1);

        // one thread created
        assert_eq!(1, result.thread_endpoints.read().unwrap().len());

        // thread had id 0
        assert_eq!(result.shutdown(), &[ThreadShutdownResponse::new(0, vec![])]);
    }
}
