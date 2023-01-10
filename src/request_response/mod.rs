mod id_targeted;

use crate::{pool_item::PoolItem, request_with_response::RequestWithResponse};
use std::fmt::Debug;

#[derive(Debug)]
pub enum RequestResponse<P, T>
where
    T: RequestWithResponse<P>,
    P: PoolItem,
{
    Request(T),
    Response(T::Response),
}

impl<P, T> PartialEq for RequestResponse<P, T>
where
    T: RequestWithResponse<P> + PartialEq,
    T::Response: PartialEq,
    P: PoolItem,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Request(l0), Self::Request(r0)) => l0 == r0,
            (Self::Response(l0), Self::Response(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<P, T> RequestResponse<P, T>
where
    T: RequestWithResponse<P>,
    P: PoolItem,
{
    pub fn request(&self) -> &T {
        let RequestResponse::Request(request) = self else {
            panic!("not expected");
        };
        request
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        samples::{Randoms, RandomsAddRequest},
        thread_request_response::AddResponse,
    };

    use super::RequestResponse;

    #[test]
    #[should_panic(expected = "not expected")]
    fn request_response_contains_response_request_panics() {
        let target = RequestResponse::<Randoms, RandomsAddRequest>::Response(AddResponse::new(
            0, true, None,
        ));

        target.request();
    }

    #[test]
    fn request_response_contains_request_request_returns_request() {
        let target = RequestResponse::Request(RandomsAddRequest(0));

        assert_eq!(&RandomsAddRequest(0), target.request());
    }
}
