use std::marker::PhantomData;

use crate::{
    pool_item::PoolItem,
    request_response::{RequestMessage, ResponseMessage},
};

use super::SenderAndReceiver;

/// A simple lightweight type that implements ThreadPoolSenderAndReceiver but throws if called
/// It is intended for use a dummy lightweight thread pool in test scenarios that don't actually use it!
#[derive(Debug, PartialEq, Eq)]
pub struct SenderAndReceiverUnimplemented<P>
where
    P: PoolItem,
{
    phantom_data: PhantomData<P>,
}

impl<P> SenderAndReceiver<P> for SenderAndReceiverUnimplemented<P>
where
    P: PoolItem,
{
    fn send_and_receive<'a, const N: usize, T, U>(
        &'a self,
        _requests: impl Iterator<Item = T> + 'a,
    ) -> Box<dyn Iterator<Item = U> + 'a>
    where
        T: RequestMessage<N, P> + 'a,
        U: ResponseMessage<N, P> + 'a,
    {
        unimplemented!(
            "this struct is intended for test scenarios that do not actually try to use the pool"
        )
    }
}

// cannot use #[derive] for this as it adds the constraint that P must also implement Default which is not necessary
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

// allow nested unimplemented SenderAndReceiverUnimplemented for when testing multiple levels of thread pools
impl<P> PoolItem for SenderAndReceiverUnimplemented<P>
where
    P: PoolItem,
{
    type Init = P::Init;
    type Api = P::Api;

    fn process_message(
        &mut self,
        _request: Self::Api,
    ) -> crate::thread_request_response::ThreadRequestResponse<Self> {
        unimplemented!(
            "this struct is intended for test scenarios that do not actually try to use the pool"
        )
    }

    fn new_pool_item(_request: Self::Init) -> Result<Self, crate::pool_item::NewPoolItemError> {
        unimplemented!(
            "this struct is intended for test scenarios that do not actually try to use the pool"
        )
    }
}
