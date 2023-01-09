use std::fmt::Debug;

use crate::{pool_item::PoolItem, thread_request_response::ThreadRequestResponse};

/// This trait allows for the pairing of requests and responses
///
/// Implementing this trait for the apis request/response pairs allows the messaging
/// infrastructure to guarantee the correctness of the messages sent and received
/// by leveraging the type system.
pub trait RequestWithResponse<P>: Debug + Into<ThreadRequestResponse<P>>
where
    P: PoolItem,
    Self::Response: Debug + From<ThreadRequestResponse<P>> + Into<ThreadRequestResponse<P>>,
{
    type Response;
}
