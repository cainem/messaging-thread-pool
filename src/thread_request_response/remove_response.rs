use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq)]
pub struct RemoveResponse {
    id: usize,
    success: bool,
}

impl RemoveResponse {
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

impl IdTargeted for RemoveResponse {
    fn id(&self) -> usize {
        todo!()
    }
}

impl<T> From<RemoveResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: RemoveResponse) -> Self {
        ThreadRequestResponse::RemovePoolItem(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for RemoveResponse
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
