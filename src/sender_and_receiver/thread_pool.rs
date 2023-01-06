use crate::{pool_item::PoolItem, request_response_2::RequestWithResponse, ThreadPool};

use super::SenderAndReceiver;

/// An implementation of the [`ThreadPoolSenderAndReceiver`] trait for [`ThreadPool`]
impl<P> SenderAndReceiver<P> for ThreadPool<P>
where
    P: PoolItem,
{
    fn send_and_receive<'a, T>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Box<dyn Iterator<Item = T::Response> + 'a>
    where
        T: RequestWithResponse<P> + 'a,
    {
        Box::new(self.send_and_receive(requests))
    }
}
