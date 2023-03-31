use crossbeam_channel::{bounded, unbounded, SendError};
use tracing::instrument;

use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_with_response::RequestWithResponse,
    sender_couplet::SenderCouplet, thread_request_response::ThreadRequestResponse, ThreadPool,
};

impl<P> ThreadPool<P>
where
    P: PoolItem,
{
    /// This function sends a request to a worker thread and receives a response back
    ///
    /// The request is received as a vec and the responses are received back in a vec
    #[instrument(skip(self, requests))]
    pub fn send_and_receive<T>(
        &self,
        requests: impl Iterator<Item = T>,
    ) -> Result<impl Iterator<Item = T::Response>, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted,
    {
        let (return_back_to, receive_from_worker) = unbounded::<ThreadRequestResponse<P>>();
        self.send(return_back_to, requests)?;
        Ok(self.receive::<T>(receive_from_worker))
    }
}

#[cfg(test)]
mod tests {
    use crate::{samples::*, thread_request_response::*, ThreadPool};

    #[test]
    fn two_threads_three_echoes_receives_expected_response() {
        let target = ThreadPool::<Randoms>::new(2);

        let requests = (0..3usize).map(|i| ThreadEchoRequest::new(i, format!("ping {i}")));

        let results: Vec<ThreadEchoResponse> = target.send_and_receive(requests).unwrap().collect();

        assert_eq!(results.len(), 3);

        assert!(results.contains(&ThreadEchoResponse::new(0, "ping 0".to_string(), 0)));
        assert!(results.contains(&ThreadEchoResponse::new(1, "ping 1".to_string(), 1)));
        assert!(results.contains(&ThreadEchoResponse::new(2, "ping 2".to_string(), 0)));
    }

    #[test]
    fn single_thread_single_init_receives_expected_response() {
        let target = ThreadPool::<Randoms>::new(1);

        let requests = (0..1).map(RandomsAddRequest);

        let result: Vec<AddResponse> = target.send_and_receive(requests).unwrap().collect();

        assert_eq!(result.len(), 1);
        assert_eq!(0, result[0].id());
    }
}
