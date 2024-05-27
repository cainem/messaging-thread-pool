use crate::{pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovePoolItemResponse {
    id: u64,
    item_existed: bool,
}

impl RemovePoolItemResponse {
    pub fn new(id: u64, item_existed: bool) -> Self {
        Self { id, item_existed }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn item_existed(&self) -> bool {
        self.item_existed
    }
}

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
        let ThreadRequestResponse::RemovePoolItem(RequestResponse::Response(response)) = response
        else {
            panic!("not expected");
        };
        response
    }
}
