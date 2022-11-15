use crate::{
    element::request_response_pair::RequestResponse, id_targeted::IdTargeted, pool_item::PoolItem,
};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq)]
pub struct RemoveResponse {
    id: u64,
    success: bool,
}

impl RemoveResponse {
    pub fn new(id: u64, success: bool) -> Self {
        Self { id, success }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn success(&self) -> bool {
        self.success
    }
}

impl IdTargeted for RemoveResponse {
    fn id(&self) -> u64 {
        todo!()
    }
}

impl<T> From<RemoveResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: RemoveResponse) -> Self {
        ThreadRequestResponse::RemoveElement(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for RemoveResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::RemoveElement(RequestResponse::Response(response)) = response else {
            panic!("not expected");
        };
        response
    }
}
