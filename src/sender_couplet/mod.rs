mod get_return_to;
mod get_thread_request;

use crossbeam_channel::Sender;

use crate::{element::Element, thread_request::ThreadRequest, thread_response::ThreadResponse};

/// This struct represents a request made to a thread pool.
/// It needs two 2 pieces of information
/// 1) It needs the request itself
/// 2) It needs a sender to return the response to
#[derive(Debug)]
pub struct SenderCouplet<E>
where
    E: Element,
{
    return_to: Sender<ThreadResponse<E::Response>>,
    request: ThreadRequest<E::Request>,
}

impl<E> SenderCouplet<E>
where
    E: Element,
{
    /// Creates a new SenderCouplet
    pub fn new<T>(return_to: Sender<ThreadResponse<E::Response>>, request: T) -> Self
    where
        T: Into<ThreadRequest<E::Request>>,
    {
        Self {
            return_to,
            request: request.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;

    use crate::{
        samples::randoms::{
            randoms_request::RandomsRequest, randoms_response::RandomsResponse, Randoms,
        },
        thread_request::ThreadRequest,
        thread_response::ThreadResponse,
    };

    use super::SenderCouplet;

    #[test]
    fn new_constructs_as_expected() {
        let (return_to, _) = unbounded::<ThreadResponse<RandomsResponse>>();
        let request = ThreadRequest::<RandomsRequest>::ThreadShutdown(0);

        let result = SenderCouplet::<Randoms>::new(return_to, request.clone());

        assert_eq!(request, result.request);
    }
}
