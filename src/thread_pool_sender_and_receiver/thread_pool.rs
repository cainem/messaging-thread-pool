use crate::{
    pool_item::PoolItem,
    request_response::{RequestMessage, ResponseMessage},
    ThreadPool,
};

use super::ThreadPoolSenderAndReceiver;

/// An implementation of the [`ThreadPoolSenderAndReceiver`] trait for [`ThreadPool`]
impl<P> ThreadPoolSenderAndReceiver<P> for ThreadPool<P>
where
    P: PoolItem,
{
    fn send_and_receive<'a, const N: usize, T, U>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Box<dyn Iterator<Item = U> + 'a>
    where
        T: RequestMessage<N, P> + 'a,
        U: ResponseMessage<N, P> + 'a,
    {
        Box::new(self.send_and_receive(requests))
    }
}
