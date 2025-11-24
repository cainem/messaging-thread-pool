use crossbeam_channel::{SendError, Sender};
use tracing::{Level, event, instrument};

use crate::{
    ThreadPool, id_targeted::IdTargeted, pool_item::PoolItem,
    request_with_response::RequestWithResponse, sender_couplet::SenderCouplet,
    thread_request_response::ThreadRequestResponse,
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
    #[instrument(skip(self, send_back_to, requests), fields(name=P::name()))]
    pub(super) fn send<T>(
        &self,
        send_back_to: Sender<ThreadRequestResponse<P>>,
        requests: impl Iterator<Item = T>,
    ) -> Result<usize, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted,
    {
        let thread_count = self
            .thread_endpoints
            .read()
            .expect("no poisoned locks")
            .len();

        let guard = self.thread_endpoints.read().expect("no poisoned locks");

        let mut request_count = 0;
        for request in requests {
            // route to correct thread; share the load based on id and the mod of the thread count
            let targeted = P::id_thread_router(request.id(), thread_count);
            event!(
                Level::DEBUG,
                "Sending to target=[{}-{}], id=[{}], message type=[{}]",
                P::name(),
                targeted,
                request.id(),
                std::any::type_name::<T>()
            );
            event!(Level::TRACE, ?request);
            guard[targeted as usize].send(&send_back_to.clone(), request)?;
            request_count += 1;
        }

        Ok(request_count)
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;

    use crate::{ThreadPool, samples::*, thread_request_response::*};

    #[test]
    fn pool_with_one_threads_send_two_echo_requests_both_processed_by_thread_0() {
        let target = ThreadPool::<Randoms>::new(1);

        let (send_back_to, receive_from_thread) = unbounded::<ThreadRequestResponse<Randoms>>();

        let requests = (0..2u64).map(|i| ThreadEchoRequest::new(i, "ping".to_string()));

        target.send(send_back_to, requests).unwrap();

        let mut responses = Vec::<ThreadEchoResponse>::new();

        for r in receive_from_thread {
            responses.push(r.into());
        }

        assert!(responses.contains(&ThreadEchoResponse::new(0, "ping".to_string(), 0)));
        assert!(responses.contains(&ThreadEchoResponse::new(1, "ping".to_string(), 0)));
    }

    #[test]
    fn pool_with_two_threads_sends_echo_requests_echo_requests_processed_by_thread_0_and_thread_1()
    {
        let target = ThreadPool::<Randoms>::new(2);

        let (send_back_to, receive_from_thread) = unbounded::<ThreadRequestResponse<Randoms>>();

        let requests = (0..2u64).map(|i| ThreadEchoRequest::new(i, "ping2".to_string()));

        target.send(send_back_to, requests).unwrap();

        let mut responses = Vec::<ThreadEchoResponse>::new();

        for r in receive_from_thread {
            responses.push(r.into());
        }

        assert!(responses.contains(&ThreadEchoResponse::new(0, "ping2".to_string(), 0)));
        assert!(responses.contains(&ThreadEchoResponse::new(1, "ping2".to_string(), 1)));
    }

    #[test]
    fn pool_with_single_thread_sends_echo_request_echo_request_processed_by_thread_0() {
        let target = ThreadPool::<Randoms>::new(1);

        let (send_back_to, receive_from_thread) = unbounded::<ThreadRequestResponse<Randoms>>();

        let requests = (0..1u64).map(|i| ThreadEchoRequest::new(i, "ping".to_string()));

        target.send(send_back_to, requests).unwrap();

        let thread_echo_response: ThreadEchoResponse = receive_from_thread.recv().unwrap().into();

        assert_eq!(
            ThreadEchoResponse::new(0, "ping".to_string(), 0),
            thread_echo_response
        )
    }
}
