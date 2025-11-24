use crate::{pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

/// This struct is returned in response to a request to add a pool item to the thread pool
/// The success field indicates that the pool item was successfully constructed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddResponse {
    id: u64,
    result: Result<u64, String>,
}

impl AddResponse {
    pub fn new(id: u64, result: Result<u64, String>) -> Self {
        assert!(
            result.is_err() || result.clone().unwrap() == id,
            "id in the success result must match the result"
        );
        Self { id, result }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn result(&self) -> Result<&u64, &String> {
        self.result.as_ref()
    }
}

impl<P> From<AddResponse> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn from(response: AddResponse) -> Self {
        ThreadRequestResponse::AddPoolItem(RequestResponse::<P, P::Init>::Response(response))
    }
}

impl<P> From<ThreadRequestResponse<P>> for AddResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::AddPoolItem(RequestResponse::<P, P::Init>::Response(
            response,
        )) = response
        else {
            panic!("unexpected")
        };
        response
    }
}
