use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
};

use super::{ThreadRequestResponse, THREAD_ABORT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadAbortResponse(pub usize);

impl RequestResponseMessage<THREAD_ABORT, false> for ThreadAbortResponse {}

impl IdTargeted for ThreadAbortResponse {
    fn id(&self) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::id_targeted::IdTargeted;

    use super::ThreadAbortResponse;

    #[test]
    fn id_2_id_returns_2() {
        let target = ThreadAbortResponse(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn id_1_id_returns_1() {
        let target = ThreadAbortResponse(1);

        assert_eq!(1, target.id());
    }
}
