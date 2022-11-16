use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response_pair::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq)]
pub struct AddResponse {
    id: u64,
    success: bool,
}

impl AddResponse {
    pub fn new(id: u64, success: bool) -> Self {
        Self { id, success }
    }
}

impl IdTargeted for AddResponse {
    fn id(&self) -> u64 {
        todo!()
    }
}

impl<T> From<AddResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: AddResponse) -> Self {
        ThreadRequestResponse::AddElement(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for AddResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::AddElement(RequestResponse::Response::<P::Init, AddResponse>(response)) = response else {
            panic!("unexpected")
        };
        response
    }
}
