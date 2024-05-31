use crate::{IdTargeted, PoolItem, ThreadRequestResponse};

impl<P> ThreadRequestResponse<P>
where
    P: PoolItem,
{
    pub fn id(&self) -> u64 {
        match self {
            ThreadRequestResponse::ThreadShutdown(request_response) => request_response.id(),
            ThreadRequestResponse::ThreadAbort(request_response) => request_response.id(),
            ThreadRequestResponse::ThreadEcho(request_response) => request_response.id(),
            ThreadRequestResponse::AddPoolItem(request_response) => request_response.id(),
            ThreadRequestResponse::RemovePoolItem(request_response) => request_response.id(),
            ThreadRequestResponse::MessagePoolItem(pool_item_api) => pool_item_api.id(),
        }
    }
}
