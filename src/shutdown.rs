use crossbeam_channel::bounded;

use crate::{
    pool_item::PoolItem, request_response_2::RequestResponse2, thread_request_response::*,
    ThreadPool,
};

impl<P> ThreadPool<P>
where
    P: PoolItem,
{
    /// This function requests that the thread pool shutdowns
    /// It sends the shutdown message to each of it's contained PoolThreads
    /// The sending of this message should cause the message loop to exit and the thread to end
    pub fn shutdown(&self) -> Vec<ThreadShutdownResponse> {
        let (send_to_pool, receive_back_from) = bounded::<ThreadRequestResponse<P>>(0);

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
            endpoint.send(&send_to_pool, ThreadShutdownRequest(id));

            let mut child_threads = Vec::<ThreadShutdownResponse>::default();

            // verify the response back from the message loop; the message loop should now have ended
            match receive_back_from
                .recv()
                .expect("the single response to the shutdown request")
            {
                ThreadRequestResponse::ThreadShutdown(RequestResponse2::Response(
                    thread_shutdown_payload,
                )) => {
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
    use std::iter;

    use crate::{samples::*, thread_request_response::*, ThreadPool};

    #[test]
    fn todo() {
        todo!();
    }

    // #[test]
    // fn two_threads_each_containing_a_sample_element_shutdown_simulates_child_thread_shutdown() {
    //     let target = ThreadPool::<Randoms>::new(2);

    //     // two thread created
    //     assert_eq!(2, target.thread_endpoints.read().unwrap().len());

    //     // adds two Randoms to the thread pool
    //     let _: Vec<AddResponse> = target
    //         .send_and_receive(
    //             iter::once(RandomsAddRequest(0)).chain(iter::once(RandomsAddRequest(1))),
    //         )
    //         .collect();

    //     // thread had id 0, 1
    //     assert_eq!(
    //         target.shutdown(),
    //         &[
    //             ThreadShutdownResponse::new(0, vec![ThreadShutdownResponse::new(0, vec![])]),
    //             ThreadShutdownResponse::new(1, vec![ThreadShutdownResponse::new(1, vec![])]),
    //         ]
    //     );
    // }

    // #[test]
    // fn two_threads_clean_shutdown_as_expected() {
    //     let result = ThreadPool::<Randoms>::new(2);

    //     // two threads created
    //     assert_eq!(2, result.thread_endpoints.read().unwrap().len());

    //     // thread had id 0, 1
    //     assert_eq!(
    //         result.shutdown(),
    //         &[
    //             ThreadShutdownResponse::new(0, vec![]),
    //             ThreadShutdownResponse::new(1, vec![])
    //         ]
    //     );
    // }

    // #[test]
    // fn single_thread_clean_shutdown_as_expected() {
    //     let result = ThreadPool::<Randoms>::new(1);

    //     // one thread created
    //     assert_eq!(1, result.thread_endpoints.read().unwrap().len());

    //     // thread had id 0
    //     assert_eq!(result.shutdown(), &[ThreadShutdownResponse::new(0, vec![])]);
    // }
}
