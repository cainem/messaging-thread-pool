mod sender_and_receiver_mock;
pub mod sender_and_receiver_raw_mock;
mod thread_pool;

use std::iter;

use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_with_response::RequestWithResponse,
    sender_couplet::SenderCouplet,
};

use crossbeam_channel::SendError;
pub use sender_and_receiver_mock::SenderAndReceiverMock;

/// This trait allows a consumer to use a trait instead of the concrete implementation of thread pool.\\
/// Unfortunately the send_and_receive are not a precise match for corresponding function in [`crate::ThreadPool`] itself.
/// This is because of the limitation of the trait return types (it has to return a boxed iterator)
pub trait SenderAndReceiver<P>
where
    P: PoolItem,
{
    /// This function sends a request to a worker thread and receives a response back
    ///
    /// The request is received as a vec and the responses are received back in a vec
    fn send_and_receive<'a, T>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Result<Box<dyn Iterator<Item = T::Response> + 'a>, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted + 'a;

    /// a default convenience function for dealing with the case when there is specifically
    /// only one message to send and (therefore) only one response to receive
    fn send_and_receive_one<'a, T>(
        &'a self,
        request: T,
    ) -> Result<T::Response, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted + 'a,
    {
        let mut responses = self.send_and_receive(iter::once(request))?;

        let Some(response) = responses.next() else {
            // not sure that this is even possible; surely the send_and_receive blocks until something is received
            panic!("response not received");
        };

        assert!(
            responses.next().is_none(),
            "more than one response received"
        );

        Ok(response)
    }
}

/// This trait is useful when multiple levels are thread pools are used and each thread pool
/// needs to be send and sync in order to be sent through the levels
pub trait ThreadSafeSenderAndReceiver<P>: SenderAndReceiver<P> + Send + Sync
where
    P: PoolItem,
{
}

#[cfg(test)]
mod tests {
    use crate::{
        samples::{MeanRequest, MeanResponse, Randoms},
        SenderAndReceiver, SenderAndReceiverMock,
    };

    #[test]
    fn send_and_receive_one_functions_as_expected() {
        let expected_response = MeanResponse { id: 1, mean: 10 };
        let target = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
            vec![MeanRequest(1)],
            vec![expected_response.clone()],
        );

        let request = MeanRequest(1);

        let Ok(response) = target.send_and_receive_one(request) else {
            panic!("not ok")
        };

        assert_eq!(expected_response, response);
    }
}
