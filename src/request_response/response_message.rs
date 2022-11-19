use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, thread_request_response::ThreadRequestResponse,
};

use super::request_response_message::RequestResponseMessage;

pub trait ResponseMessage<const N: usize, P>:
    RequestResponseMessage<N, false> + From<ThreadRequestResponse<P>> + IdTargeted
where
    P: PoolItem,
{
}

impl<const N: usize, T, P> ResponseMessage<N, P> for T
where
    P: PoolItem,
    T: RequestResponseMessage<N, false>,
    T: From<ThreadRequestResponse<P>>,
{
}
