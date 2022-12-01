pub mod thread_pool;
pub mod thread_pool_mock;

use crate::{
    pool_item::PoolItem,
    request_response::{request_message::RequestMessage, response_message::ResponseMessage},
};

/// This trait allows a consumer to use a trait instead of the concrete implementation of thread pool.\\
/// Unfortunately the send_and_receive are not a precise match for corresponding function in [`super::ThreadPool`] itself.
/// This is because of the limitation of the trait return types (it has to return a boxed iterator)
pub trait ThreadPoolSenderAndReceiver<P>
where
    P: PoolItem,
{
    /// This function sends a request to a worker thread and receives a response back
    ///
    /// The request is received as a vec and the responses are received back in a vec
    fn send_and_receive<'a, const N: usize, T, U>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Box<dyn Iterator<Item = U> + 'a>
    where
        T: RequestMessage<N, P> + 'a,
        U: ResponseMessage<N, P> + 'a;
}
