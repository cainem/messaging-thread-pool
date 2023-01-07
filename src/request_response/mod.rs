mod id_targeted;

use crate::{pool_item::PoolItem, thread_request_response::ThreadRequestResponse};
use std::fmt::Debug;

pub trait RequestWithResponse<P>: Debug + Into<ThreadRequestResponse<P>>
where
    P: PoolItem,
    Self::Response: Debug + From<ThreadRequestResponse<P>> + Into<ThreadRequestResponse<P>>,
{
    type Response;
}

#[derive(Debug, PartialEq)]
pub enum RequestResponse<P, T>
where
    T: RequestWithResponse<P>,
    P: PoolItem,
{
    Request(T),
    Response(T::Response),
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
