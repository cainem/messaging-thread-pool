use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

/// This struct is returned in response to a request to add a pool item to the thread pool
/// The success field indicates that the pool item was successfully constructed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddResponse {
    id: usize,
    result: Result<(), String>,
}

impl AddResponse {
    pub fn new(id: usize, result: Result<(), String>) -> Self {
        Self { id, result }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn result(&self) -> Result<&(), &String> {
        self.result.as_ref()
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
        ThreadRequestResponse::AddPoolItem(RequestResponse::<P, P::Init>::Response(response))
    }
}

impl<P> From<ThreadRequestResponse<P>> for AddResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::AddPoolItem(RequestResponse::<P, P::Init>::Response(response)) = response else {
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
        let target = AddResponse::new(2, Ok(()));

        assert_eq!(2, target.id());
    }

    #[test]
    fn id_1_id_returns_1() {
        let target = AddResponse::new(1, Ok(()));

        assert_eq!(1, target.id());
    }
}
