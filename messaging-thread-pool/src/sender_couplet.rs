use crossbeam_channel::{SendError, Sender, unbounded};

use crate::{
    pool_item::PoolItem,
    request_with_response::RequestWithResponse,
    thread_request_response::{ThreadAbortRequest, ThreadRequestResponse},
};

/// A struct that defines the contents of a message sent to the thread pool.
#[derive(Debug)]
pub struct SenderCouplet<P>
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
    /// Creates a new SenderCouplet.
    pub fn new<T>(return_to: Sender<ThreadRequestResponse<P>>, request: T) -> Self
    where
        T: RequestWithResponse<P>,
    {
        Self {
            return_to,
            request: request.into(),
        }
    }

    /// Returns the request contained in the couplet.
    pub fn request(&self) -> &ThreadRequestResponse<P> {
        &self.request
    }

    /// Returns the channel to return the response to.
    #[allow(dead_code)]
    pub fn return_to(&self) -> &Sender<ThreadRequestResponse<P>> {
        &self.return_to
    }
}

pub(crate) fn thread_abort_send_error<P>(request_id: u64) -> SendError<SenderCouplet<P>>
where
    P: PoolItem,
{
    let (return_to, _) = unbounded::<ThreadRequestResponse<P>>();
    SendError(SenderCouplet::new(
        return_to,
        ThreadAbortRequest(request_id),
    ))
}
