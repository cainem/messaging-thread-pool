mod sender_and_receiver_mock;
mod thread_pool;

use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_with_response::RequestWithResponse,
};

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
    ) -> Box<dyn Iterator<Item = T::Response> + 'a>
    where
        T: RequestWithResponse<P> + IdTargeted + 'a;
}

/// This trait is useful when multiple levels are thread pools are used and each thread pool
/// needs to be send and sync in order to be sent through the levels
pub trait ThreadSafeSenderAndReceiver<P>: SenderAndReceiver<P> + Send + Sync
where
    P: PoolItem,
{
}
