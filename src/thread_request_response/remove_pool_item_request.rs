use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse,
    request_with_response::RequestWithResponse,
};

use super::{RemovePoolItemResponse, ThreadRequestResponse};

/// A request to remove a pool item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovePoolItemRequest(pub u64);

impl IdTargeted for RemovePoolItemRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

impl<P> RequestWithResponse<P> for RemovePoolItemRequest
where
    P: PoolItem,
{
    type Response = RemovePoolItemResponse;
}

impl<P> From<RemovePoolItemRequest> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn from(request: RemovePoolItemRequest) -> Self {
        ThreadRequestResponse::RemovePoolItem(RequestResponse::<P, RemovePoolItemRequest>::Request(
            request,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted,
        thread_request_response::remove_pool_item_request::RemovePoolItemRequest,
    };

    #[test]
    fn request_id_2_id_returns_2() {
        let target = RemovePoolItemRequest(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_id_1_id_returns_1() {
        let target = RemovePoolItemRequest(1);

        assert_eq!(1, target.id());
    }
}
