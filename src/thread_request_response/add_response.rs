use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
};

use super::{ThreadRequestResponse, ADD_POOL_ITEM};

/// This struct is returned in response to a request to add a pool item to the thread pool
/// The success field indicates that the pool item was successfully constructed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddResponse {
    id: usize,
    success: bool,
    error_message: Option<String>,
}

impl AddResponse {
    pub fn new(id: usize, success: bool, error_message: Option<String>) -> Self {
        Self {
            id,
            success,
            error_message,
        }
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }
}

impl RequestResponseMessage<ADD_POOL_ITEM, false> for AddResponse {}

impl IdTargeted for AddResponse {
    fn id(&self) -> usize {
        self.id
    }
}

impl<T> From<AddResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: AddResponse) -> Self {
        ThreadRequestResponse::AddPoolItem(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for AddResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::AddPoolItem(RequestResponse::Response::<ADD_POOL_ITEM, P::Init, AddResponse>(response)) = response else {
            panic!("unexpected")
        };
        response
    }
}

#[cfg(test)]
mod tests {
    use crate::id_targeted::IdTargeted;

    use super::AddResponse;

    #[test]
    fn id_2_id_returns_2() {
        let target = AddResponse::new(2, true, None);

        assert_eq!(2, target.id());
    }

    #[test]
    fn id_1_id_returns_1() {
        let target = AddResponse::new(1, true, None);

        assert_eq!(1, target.id());
    }
}
