use crossbeam_channel::SendError;

use crate::{
    ThreadPool, id_targeted::IdTargeted, pool_item::PoolItem,
    request_with_response::RequestWithResponse, sender_couplet::SenderCouplet,
};

use super::SenderAndReceiver;

/// An implementation of the [`ThreadPoolSenderAndReceiver`] trait for [`ThreadPool`]
impl<P> SenderAndReceiver<P> for ThreadPool<P>
where
    P: PoolItem,
{
    fn send_and_receive<'a, T>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Result<Box<dyn Iterator<Item = T::Response> + 'a>, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted + 'a,
    {
        match self.send_and_receive(requests) {
            Ok(result) => Ok(Box::new(result)),
            Err(err) => Err(err),
        }
    }
}
