use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, thread_request_response::ThreadRequestResponse,
};
use std::fmt::Debug;

pub trait RequestWithResponse<P>:
    IdTargeted + Debug + PartialEq + Into<ThreadRequestResponse<P>>
where
    P: PoolItem,
    Self::Response:
        Debug + PartialEq + From<ThreadRequestResponse<P>> + Into<ThreadRequestResponse<P>>,
{
    type Response;
}

#[derive(Debug, PartialEq)]
pub enum RequestResponse2<P, T>
where
    T: RequestWithResponse<P>,
    P: PoolItem,
{
    Request(T),
    Response(T::Response),
}
