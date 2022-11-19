use crossbeam_channel::Sender;

use crate::{
    pool_item::PoolItem, request_response::request_message::RequestMessage,
    sender_couplet::SenderCouplet, thread_request_response::ThreadRequestResponse,
};

use super::ThreadEndpoint;

impl<P> ThreadEndpoint<P>
where
    P: PoolItem,
{
    /// This function send an asynchronous request to a thread pool
    pub fn send<const N: usize, T>(&self, sender: &Sender<ThreadRequestResponse<P>>, request: T)
    where
        T: RequestMessage<N, P>,
    {
        self.sender
            .send(SenderCouplet::<P>::new(sender.clone(), request.into()))
            .expect("The receiver thread to always be available");
    }
}

#[cfg(test)]
mod tests {
    use std::thread::spawn;

    use crossbeam_channel::unbounded;

    use crate::{samples::*, thread_endpoint::ThreadEndpoint};

    #[test]
    fn todo() {
        todo!();
    }

    // #[test]
    // fn pass_echo_message_through_echo_message_received_at_other_end_of_channel() {
    //     let echo_request = ThreadRequest::<RandomsRequest>::ThreadEcho(0, "hello".to_string());

    //     // create a thread (which instantly terminates) purely for its join_handle
    //     let join_handle = spawn(|| 1);

    //     // create channels to send and receive responses
    //     let (to_thread_sender, receiver_from_endpoint) = unbounded::<SenderCouplet<Randoms>>();
    //     let (to_endpoint, from_thread) = unbounded::<ThreadResponse<RandomsResponse>>();

    //     let target = ThreadEndpoint {
    //         sender: to_thread_sender,
    //         join_handle: join_handle,
    //     };

    //     // call send
    //     target.send(&to_endpoint, echo_request.clone());

    //     // get the message sent
    //     let sender_couplet = receiver_from_endpoint.recv().unwrap();

    //     // confirm that it is in the expected form; it is difficult to confirm the correct sender was sent
    //     assert_eq!(sender_couplet.get_thread_request(), &echo_request);

    //     // create and send a response message
    //     let response = ThreadResponse::<RandomsResponse>::ThreadEcho(0, 0, "hello".to_string());
    //     sender_couplet
    //         .get_return_to()
    //         .send(response.clone())
    //         .unwrap();
    //     let response_result = from_thread.recv().unwrap();

    //     // confirm that the message received is as expected
    //     assert_eq!(response_result, response);

    //     // join back to the thread
    //     target.join_handle.join().unwrap();
    // }
}
