use crossbeam_channel::Sender;

use crate::{pool_item::PoolItem, thread_request_response::ThreadRequestResponse};

#[derive(Debug)]
pub struct SenderCouplet<P>
where
    P: PoolItem,
{
    return_to: Sender<ThreadRequestResponse<P>>,
    request: ThreadRequestResponse<P>,
}

impl<P> SenderCouplet<P>
where
    P: PoolItem,
{
    /// Creates a new SenderCouplet
    pub fn new<T>(return_to: Sender<ThreadRequestResponse<P>>, request: T) -> Self
    where
        T: Into<ThreadRequestResponse<P>>,
    {
        Self {
            return_to,
            request: request.into(),
        }
    }

    pub fn request(&self) -> &ThreadRequestResponse<P> {
        debug_assert!(self.request.is_request());
        &self.request
    }

    pub fn return_to(&self) -> &Sender<ThreadRequestResponse<P>> {
        &self.return_to
    }
}
