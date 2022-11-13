use crossbeam_channel::Sender;

use crate::{pool_item::PoolItem, thread_request_response::ThreadRequestResponse};

#[derive(Debug)]
pub struct SenderCouplet2<E>
where
    E: PoolItem,
{
    return_to: Sender<ThreadRequestResponse<E>>,
    request: ThreadRequestResponse<E>,
}

impl<E> SenderCouplet2<E>
where
    E: PoolItem,
{
    /// Creates a new SenderCouplet
    pub fn new<T>(return_to: Sender<ThreadRequestResponse<E>>, request: T) -> Self
    where
        T: Into<ThreadRequestResponse<E>>,
    {
        Self {
            return_to,
            request: request.into(),
        }
    }

    pub fn request(&self) -> &ThreadRequestResponse<E> {
        debug_assert!(self.request.is_request());
        &self.request
    }

    pub fn return_to(&self) -> &Sender<ThreadRequestResponse<E>> {
        &self.return_to
    }
}
