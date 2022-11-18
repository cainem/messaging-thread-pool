use std::cell::RefCell;

use crossbeam_channel::Sender;
use tracing::{event, instrument, Level};

use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem,
    request_response::request_response_message::RequestResponseMessage,
    thread_request_response::ThreadRequestResponse, ThreadPool,
};

impl<P> ThreadPool<P>
where
    P: PoolItem,
{
    /// This function sends a request to a thread within the pool
    ///
    /// The request is received as a vec
    /// The parent level bundles all of the work for the child levels into a vec of requests
    /// The work is distributed within the thread pool and returned as a vec of responses
    ///
    /// The work will be distributed based on the mod of the id of the requests target
    #[instrument(skip(self, send_back_to, requests))]
    pub(super) fn send<const N: usize, T>(
        &self,
        send_back_to: Sender<ThreadRequestResponse<P>>,
        requests: impl Iterator<Item = T>,
    ) where
        T: Into<ThreadRequestResponse<P>>,
        T: RequestResponseMessage<N, true>,
    {
        let thread_count = self
            .thread_endpoints
            .read()
            .expect("no poisoned locks")
            .len();

        let guard =  self.thread_endpoints.read().expect("no poisoned locks");

        for request in requests {
            let request: ThreadRequestResponse<P> = request.into();
            // route to correct thread; share the load based on id and the mod of the thread count
            let targeted = request.id() as usize % thread_count;
            event!(Level::DEBUG, "Sending to target {}", request.id());
            event!(Level::TRACE, ?request);
            guard[targeted].send(&send_back_to.clone(), request);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crossbeam_channel::unbounded;

    use crate::{samples::*, ThreadPool};

    #[test]
    fn todo() {
        todo!();

        let vec = vec![1, 2, 3];

        for v in vec.into_iter() {}

        for v in vec {}
    }

    // #[test]
    // fn pool_with_one_threads_send_two_echo_requests_both_processed_by_thread_0() {
    //     let target = ThreadPool::<Randoms>::new(1);

    //     let (send_back_to, receive_from_thread) = unbounded::<ThreadResponse<RandomsResponse>>();

    //     let requests: Vec<_> = (0..2u64)
    //         .map(|i| ThreadRequest::ThreadEcho(i, "ping".to_string()))
    //         .collect();
    //     let requests = RefCell::new(requests);

    //     target.send(send_back_to, &requests);

    //     let mut responses = Vec::new();

    //     for r in receive_from_thread {
    //         if let ThreadResponse::ThreadEcho(targeted, actual, s) = r {
    //             responses.push((targeted, actual, s));
    //         } else {
    //             panic!("not expected");
    //         }

    //         if responses.len() == 2 {
    //             break;
    //         }
    //     }

    //     assert!(responses.contains(&(0, 0, "ping [0]".to_string())));
    //     assert!(responses.contains(&(1, 0, "ping [0]".to_string())));
    // }

    // #[test]
    // fn pool_with_two_threads_sends_echo_requests_echo_requests_processed_by_thread_0_and_thread_1()
    // {
    //     let target = ThreadPool::<Randoms>::new(2);

    //     let (send_back_to, receive_from_thread) = unbounded::<ThreadResponse<RandomsResponse>>();

    //     let requests: Vec<_> = (0..2u64)
    //         .map(|i| ThreadRequest::ThreadEcho(i, "ping2".to_string()))
    //         .collect();
    //     let requests = RefCell::new(requests);

    //     target.send(send_back_to, &requests);

    //     let mut responses = Vec::new();

    //     for r in receive_from_thread {
    //         if let ThreadResponse::ThreadEcho(targeted, actual, s) = r {
    //             responses.push((targeted, actual, s));
    //         } else {
    //             panic!("not expected");
    //         }

    //         if responses.len() == 2 {
    //             break;
    //         }
    //     }

    //     assert!(responses.contains(&(0, 0, "ping2 [0]".to_string())));
    //     assert!(responses.contains(&(1, 1, "ping2 [1]".to_string())));
    // }

    // #[test]
    // fn pool_with_single_thread_sends_echo_request_echo_request_processed_by_thread_0() {
    //     let target = ThreadPool::<Randoms>::new(1);

    //     let (send_back_to, receive_from_thread) = unbounded::<ThreadResponse<RandomsResponse>>();

    //     let requests: Vec<_> = (0..1u64)
    //         .map(|i| ThreadRequest::ThreadEcho(i, "ping".to_string()))
    //         .collect();
    //     let requests = RefCell::new(requests);

    //     target.send(send_back_to, &requests);

    //     if let ThreadResponse::ThreadEcho(targeted, actual, s) = receive_from_thread.recv().unwrap()
    //     {
    //         // request sent to thread 0
    //         assert_eq!("ping [0]", s);
    //         assert_eq!(0, actual);
    //         assert_eq!(0, targeted);
    //     } else {
    //         panic!("not expected");
    //     }
    // }
}
