use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response_2::{RequestResponse2, RequestWithResponse},
};

use super::{ThreadRequestResponse, ThreadShutdownResponse, THREAD_SHUTDOWN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadShutdownRequest(pub usize);

impl<P> RequestWithResponse<P> for ThreadShutdownRequest
where
    P: PoolItem,
{
    type Response = ThreadShutdownResponse;
}

impl IdTargeted for ThreadShutdownRequest {
    fn id(&self) -> usize {
        self.0
    }
}

impl<P> From<ThreadShutdownRequest> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn from(request: ThreadShutdownRequest) -> Self {
        ThreadRequestResponse::ThreadShutdown(
            RequestResponse2::<P, ThreadShutdownRequest>::Request(request),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::id_targeted::IdTargeted;

    use super::ThreadShutdownRequest;

    #[test]
    fn request_id_2_id_returns_2() {
        let target = ThreadShutdownRequest(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_id_1_id_returns_1() {
        let target = ThreadShutdownRequest(1);

        assert_eq!(1, target.id());
    }
}
