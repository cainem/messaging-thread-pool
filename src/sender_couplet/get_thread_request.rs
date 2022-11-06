use crate::{element::Element, thread_request::ThreadRequest};

use super::SenderCouplet;

impl<E> SenderCouplet<E>
where
    E: Element,
{
    /// Gets the thread request of the SenderCouplet
    pub fn get_thread_request(&self) -> &ThreadRequest<E::Request> {
        &self.request
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        samples::randoms::{
            randoms_request::RandomsRequest, randoms_response::RandomsResponse, Randoms,
        },
        sender_couplet::SenderCouplet,
        thread_request::ThreadRequest,
        thread_response::ThreadResponse,
    };
    use crossbeam_channel::unbounded;

    #[test]
    fn returns_property() {
        let (return_to, _) = unbounded::<ThreadResponse<RandomsResponse>>();
        let request = ThreadRequest::<RandomsRequest>::ThreadShutdown(0);

        let target = SenderCouplet::<Randoms>::new(return_to, request.clone());

        assert_eq!(&request, target.get_thread_request());
    }
}
