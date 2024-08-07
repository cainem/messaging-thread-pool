use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse,
    request_with_response::RequestWithResponse,
};

use super::{ThreadAbortResponse, ThreadRequestResponse};

/// Request to abort a thread; for testing only
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadAbortRequest(pub u64);

impl IdTargeted for ThreadAbortRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

impl<P> RequestWithResponse<P> for ThreadAbortRequest
where
    P: PoolItem,
{
    type Response = ThreadAbortResponse;
}

impl<P> From<ThreadAbortRequest> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn from(request: ThreadAbortRequest) -> Self {
        ThreadRequestResponse::ThreadAbort(RequestResponse::Request(request))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted, thread_request_response::thread_abort_request::ThreadAbortRequest,
    };

    #[test]
    fn request_id_2_id_returns_2() {
        let target = ThreadAbortRequest(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_id_1_id_returns_1() {
        let target = ThreadAbortRequest(1);

        assert_eq!(1, target.id());
    }
}
