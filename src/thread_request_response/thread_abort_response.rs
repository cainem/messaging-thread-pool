use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadAbortResponse(pub usize);

impl IdTargeted for ThreadAbortResponse {
    fn id(&self) -> usize {
        todo!()
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

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
