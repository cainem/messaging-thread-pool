use crossbeam_channel::Sender;

use crate::{element::Element, thread_response::ThreadResponse};

use super::SenderCouplet;

impl<E> SenderCouplet<E>
where
    E: Element,
{
    /// return endpoint back to caller, so the response can be returned
    pub fn get_return_to(&self) -> &Sender<ThreadResponse<E::Response>> {
        &self.return_to
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;

    use crate::{
        samples::randoms::{
            randoms_request::RandomsRequest, randoms_response::RandomsResponse, Randoms,
        },
        sender_couplet::SenderCouplet,
        thread_request::ThreadRequest,
        thread_response::ThreadResponse,
    };

    #[test]
    fn returns_property_does_not_panic() {
        let (return_to, _) = unbounded::<ThreadResponse<RandomsResponse>>();
        let request = ThreadRequest::<RandomsRequest>::ThreadShutdown(0);

        let target = SenderCouplet::<Randoms>::new(return_to, request.clone());

        let _ = target.get_return_to();
    }
}
