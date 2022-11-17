use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq)]
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

impl IdTargeted for RemovePoolItemResponse {
    fn id(&self) -> usize {
        todo!()
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
        let ThreadRequestResponse::RemovePoolItem(RequestResponse::Response(response)) = response else {
            panic!("not expected");
        };
        response
    }
}
