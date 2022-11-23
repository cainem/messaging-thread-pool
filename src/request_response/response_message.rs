use crate::{pool_item::PoolItem, thread_request_response::ThreadRequestResponse};

use super::request_response_message::RequestResponseMessage;

/// This trait is auto-implemented for responses
pub trait ResponseMessage<const N: usize, P>:
    RequestResponseMessage<N, false> + From<ThreadRequestResponse<P>>
where
    P: PoolItem,
{
}

impl<const N: usize, T, P> ResponseMessage<N, P> for T
where
    P: PoolItem,
    T: RequestResponseMessage<N, false> + From<ThreadRequestResponse<P>>,
{
}
