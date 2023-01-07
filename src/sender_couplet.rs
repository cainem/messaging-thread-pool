use crossbeam_channel::Sender;

use crate::{
    pool_item::PoolItem, request_response::RequestWithResponse,
    thread_request_response::ThreadRequestResponse,
};

/// A struct that defines the contents of a message sent to the thread pool
#[derive(Debug)]
pub(crate) struct SenderCouplet<P>
where
    P: PoolItem,
{
    pub return_to: Sender<ThreadRequestResponse<P>>,
    pub request: ThreadRequestResponse<P>,
}

impl<P> SenderCouplet<P>
where
    P: PoolItem,
{
    /// Creates a new SenderCouplet
    pub fn new<T>(return_to: Sender<ThreadRequestResponse<P>>, request: T) -> Self
    where
        T: RequestWithResponse<P>,
    {
        Self {
            return_to,
            request: request.into(),
        }
    }

    pub fn request(&self) -> &ThreadRequestResponse<P> {
        &self.request
    }

    #[allow(dead_code)]
    pub fn return_to(&self) -> &Sender<ThreadRequestResponse<P>> {
        &self.return_to
    }
}
