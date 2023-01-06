use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response_2::RequestResponse2};

use super::ThreadRequestResponse;

/// This struct is returned in response to a request to add a pool item to the thread pool
/// The success field indicates that the pool item was successfully constructed
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

    pub fn id(&self) -> usize {
        self.id
    }
}

impl IdTargeted for AddResponse {
    fn id(&self) -> usize {
        self.id
    }
}

impl<P> From<AddResponse> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn from(response: AddResponse) -> Self {
        ThreadRequestResponse::AddPoolItem(RequestResponse2::<P, P::Init>::Response(response))
    }
}

impl<P> From<ThreadRequestResponse<P>> for AddResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::AddPoolItem(RequestResponse2::<P, P::Init>::Response(response)) = response else {
            panic!("unexpected")
        };
        response
    }
}

#[cfg(test)]
mod tests {
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
