use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response::{RequestResponse, RequestResponseMessage},
};

use super::{ThreadRequestResponse, THREAD_SHUTDOWN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadShutdownRequest(pub usize);

impl RequestResponseMessage<THREAD_SHUTDOWN, true> for ThreadShutdownRequest {}

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
        ThreadRequestResponse::ThreadShutdown(RequestResponse::Request(request))
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
