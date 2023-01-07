use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, thread_request_response::ThreadRequestResponse,
};
use std::fmt::Debug;

pub trait RequestWithResponse<P>: Debug + Into<ThreadRequestResponse<P>>
where
    P: PoolItem,
    Self::Response: Debug + From<ThreadRequestResponse<P>> + Into<ThreadRequestResponse<P>>,
{
    type Response;
}

#[derive(Debug, PartialEq)]
pub enum RequestResponse2<P, T>
where
    T: RequestWithResponse<P>,
    P: PoolItem,
{
    Request(T),
    Response(T::Response),
}

impl<P, T> RequestResponse2<P, T>
where
    T: RequestWithResponse<P>,
    P: PoolItem,
{
    pub fn request(&self) -> &T {
        let RequestResponse2::Request(request) = self else {
            panic!("not expected");
        };
        request
    }
}

impl<P, T> IdTargeted for RequestResponse2<P, T>
where
    T: RequestWithResponse<P> + IdTargeted,
    P: PoolItem,
{
    fn id(&self) -> usize {
        let RequestResponse2::Request(request) = self else {
            panic!("not expected");
        };
        request.id()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
