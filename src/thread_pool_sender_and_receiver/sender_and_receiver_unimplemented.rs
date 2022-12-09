use std::marker::PhantomData;

use crate::pool_item::PoolItem;

use super::ThreadPoolSenderAndReceiver;

/// A simple lightweight type that implements ThreadPoolSenderAndReceiver but throws if called
/// It is intended for use a dummy lightweight thread pool in test scenarios that don't actually use it!
#[derive(Debug, PartialEq)]
pub struct SenderAndReceiverUnimplemented<P>
where
    P: PoolItem,
{
    phantom_data: PhantomData<P>,
}

impl<P> ThreadPoolSenderAndReceiver<P> for SenderAndReceiverUnimplemented<P>
where
    P: PoolItem,
{
    fn send_and_receive<'a, const N: usize, T, U>(
        &'a self,
        _requests: impl Iterator<Item = T> + 'a,
    ) -> Box<dyn Iterator<Item = U> + 'a>
    where
        T: crate::request_response::RequestMessage<N, P> + 'a,
        U: crate::request_response::ResponseMessage<N, P> + 'a,
    {
        unimplemented!(
            "this struct is intended for test scenarios that do not actually try to use the pool"
        )
    }
}

// cannot use #[derive] for this as it adds the constraint that P must also implement Default
impl<P> Default for SenderAndReceiverUnimplemented<P>
where
    P: PoolItem,
{
    fn default() -> Self {
        Self {
            phantom_data: Default::default(),
        }
    }
}
