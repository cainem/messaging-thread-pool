use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, thread_request_response::ThreadRequestResponse,
};
use std::fmt::Debug;

use super::request_response_message::RequestResponseMessage;

pub trait RequestMessage<const N: usize, P>: Into<ThreadRequestResponse<P>> + IdTargeted
where
    P: PoolItem,
{
}

impl<const N: usize, T, P> RequestMessage<N, P> for T
where
    P: PoolItem,
    T: RequestResponseMessage<N, true> + Into<ThreadRequestResponse<P>> + IdTargeted + Debug,
{
}
