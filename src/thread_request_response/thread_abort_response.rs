use crate::{pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadAbortResponse(pub usize);

impl ThreadAbortResponse {
    pub fn thread_id(&self) -> usize {
        self.0
    }
}

impl<T> From<ThreadAbortResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: ThreadAbortResponse) -> Self {
        ThreadRequestResponse::ThreadAbort(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for ThreadAbortResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::ThreadAbort(RequestResponse::Response(response)) = response else {
            panic!("unexpected")
        };
        response
    }
}
