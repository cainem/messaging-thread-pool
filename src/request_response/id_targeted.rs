use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_with_response::RequestWithResponse,
};

use super::RequestResponse;

impl<P, T> IdTargeted for RequestResponse<P, T>
where
    T: RequestWithResponse<P> + IdTargeted,
    P: PoolItem,
{
    fn id(&self) -> usize {
        let RequestResponse::Request(request) = self else {
            panic!("not expected; only requests are required to support IdTargeted");
        };
        request.id()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted,
        request_response::RequestResponse,
        samples::{Randoms, RandomsAddRequest},
        thread_request_response::AddResponse,
    };

    #[test]
    #[should_panic(expected = "not expected; only requests are required to support IdTargeted")]
    fn request_response_contains_response_request_panics() {
        let target =
            RequestResponse::<Randoms, RandomsAddRequest>::Response(AddResponse::new(0, Ok(())));

        target.id();
    }

    #[test]
    fn request_response_contains_request_id_1_returns_1() {
        let target = RequestResponse::Request(RandomsAddRequest(1));

        assert_eq!(1, target.id());
    }

    #[test]
    fn request_response_contains_request_id_0_returns_0() {
        let target = RequestResponse::Request(RandomsAddRequest(0));

        assert_eq!(0, target.id());
    }
}
