use crate::{
    pool_item::PoolItem,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
};

use super::{ThreadRequestResponse, REMOVE_POOL_ITEM};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovePoolItemResponse {
    id: usize,
    success: bool,
}

impl RemovePoolItemResponse {
    pub fn new(id: usize, success: bool) -> Self {
        Self { id, success }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn success(&self) -> bool {
        self.success
    }
}

impl RequestResponseMessage<REMOVE_POOL_ITEM, false> for RemovePoolItemResponse {}

impl<T> From<RemovePoolItemResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: RemovePoolItemResponse) -> Self {
        ThreadRequestResponse::RemovePoolItem(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for RemovePoolItemResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::RemovePoolItem(RequestResponse::Response(response)) = response else {
            panic!("not expected");
        };
        response
    }
}
