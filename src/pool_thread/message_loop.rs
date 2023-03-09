use std::collections::btree_map::Entry;

use tracing::{event, instrument, Level};

use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse,
    sender_couplet::SenderCouplet, thread_request_response::*,
};

use super::PoolThread;

impl<P> PoolThread<P>
where
    P: PoolItem,
{
    /// This function forms the "message loop" of the [`PoolThread`]
    ///
    /// This is an infinite loop that pulls requests off of the [`PoolThread`]s receive channel
    /// This function will be running in the context of its own dedicated thread
    ///
    /// Messages split logically into 2 types, pool item requests and thread requests
    /// Pool requests are forwarded to the appropriate pool item (that is contained in a btree)
    /// The message can be targeted by virtue of the fact they all contain a key of the target that
    /// the message is intended for.
    ///
    /// Thread requests are handled within this loop and are used to control the thread pool
    ///
    /// ThreadShutdown and ThreadAbort messages cause the message loop to exit and as a result end the thread.
    #[instrument(skip(self), fields(id=self.thread_id, name=P::name()))]
    pub fn message_loop(&mut self) {
        // will loop until the queue is empty and the sender is dropped
        while let Ok(sender_couplet) = self.pool_thread_receiver.recv() {
            event!(
                Level::TRACE,
                "receiving request {:?}",
                sender_couplet.request(),
            );

            let SenderCouplet { return_to, request } = sender_couplet;

            let response = match request {
                ThreadRequestResponse::MessagePoolItem(request) => {
                    let id = request.id();

                    // find the pool item that needs to process the request
                    let response = if let Some(targeted) = self.pool_item_map.get_mut(&id) {
                        // give the opportunity to add pool item tracing
                        let guards = P::add_pool_item_tracing(targeted);
                        // process the message
                        let response = targeted.process_message(request);

                        drop(guards);
                        response
                    } else {
                        P::id_not_found(&request)
                    };

                    response
                }
                ThreadRequestResponse::AddPoolItem(RequestResponse::Request(request)) => {
                    let id = request.id();

                    match P::new_pool_item(request) {
                        Ok(new_pool_item) => {
                            event!(
                                Level::DEBUG,
                                "Inserting a new {:?} into the threads map",
                                P::name()
                            );

                            // try and add the new item
                            match self.pool_item_map.entry(id) {
                                Entry::Vacant(v) => {
                                    v.insert(new_pool_item);
                                    AddResponse::new(id, Ok(id))
                                }
                                Entry::Occupied(_) => AddResponse::new(
                                    id,
                                    Err("failed to add; pool item already exists".to_string()),
                                ),
                            }
                        }
                        Err(new_pool_item_error) => {
                            AddResponse::new(id, Err(new_pool_item_error.error_message))
                        }
                    }
                    .into()
                }
                ThreadRequestResponse::RemovePoolItem(RequestResponse::Request(request)) => {
                    let id = request.id();
                    let success = self.pool_item_map.remove(&id).is_some();
                    RemovePoolItemResponse::new(id, success).into()
                }
                ThreadRequestResponse::ThreadShutdown(RequestResponse::Request(request)) => {
                    let id = request.id();
                    debug_assert_eq!(
                        self.thread_id, id,
                        "this messages should have targeted this thread"
                    );
                    // this call to shutdown the child threads and consequently empty the internal map
                    // is how thread shutdown differs from thread abort. Abort just exist the loop and leaves the
                    // state in place
                    let children = self.shutdown_child_pool();
                    return_to
                        .send(ThreadShutdownResponse::new(id, children).into())
                        .expect("the send should always succeed");
                    debug_assert!(
                        self.pool_item_map.is_empty(),
                        "ThreadShutdown should drain all elements"
                    );
                    // return breaking out of the message loop and thus ending the thread.
                    return;
                }
                ThreadRequestResponse::ThreadEcho(RequestResponse::Request(request)) => {
                    ThreadEchoResponse::new(
                        request.id(),
                        request.message().to_string(),
                        self.thread_id,
                    )
                    .into()
                }
                ThreadRequestResponse::ThreadAbort(RequestResponse::Request(request)) => {
                    let id = request.id();
                    debug_assert_eq!(
                        self.thread_id, id,
                        "this messages should have targeted this thread"
                    );

                    return_to
                        .send(ThreadAbortResponse(id).into())
                        .expect("the send should always succeed");

                    // return breaking out of the message loop and thus ending the thread.
                    return;
                }

                _ => panic!("unrecognised thread thread request"),
            };
            event!(Level::TRACE, ?response);

            match return_to.send(response) {
                Ok(_) => (),
                Err(err) => {
                    // The channel that is supposed to be receiving the response cannot receive it
                    // It has probably been dropped
                    // discard the response message and continue
                    event!(Level::WARN, "Cannot return results, other end of channel has most likely been dropped. Err = {}", &err);
                }
            }

            // loop will only exit here if the "main" thread has exited; this is not expected
        }

        // to get here the "send end" of the channel must have been dropped which
        // suggest that the main thread has ended.
        panic!("message loop finished unexpectedly; thread shutting down");
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;

    use crate::{
        pool_thread::PoolThread, samples::*, sender_couplet::SenderCouplet,
        thread_request_response::*,
    };

    #[test]
    fn send_init_id_2_twice_returns_response_indicating_second_request_was_ignored() {
        let id = 2;
        let init_request = RandomsAddRequest(id);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(3, request_receive);

        // send the init request twice
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                init_request.clone(),
            ))
            .unwrap();

        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadAbortRequest(3)))
            .unwrap();

        target.message_loop();

        let response_0: AddResponse = response_receive.recv().unwrap().into();
        let response_1: AddResponse = response_receive.recv().unwrap().into();

        assert_eq!(2, response_0.id());
        assert!(response_0.result().is_ok());
        assert_eq!(1, target.pool_item_map.len());
        assert_eq!(2, target.pool_item_map.get(&id).unwrap().id);

        assert_eq!(2, response_1.id());
        assert!(response_1.result().is_err());
        assert_eq!(
            "failed to add; pool item already exists",
            response_1.result().err().unwrap()
        )
    }

    #[test]
    fn send_remove_element_with_id_12_expected_element_removed_from_map_set() {
        let id = 12;
        let init_request = RandomsAddRequest(id);

        let remove_pool_item_request = RemovePoolItemRequest(id);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();

        // send the remove request
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                remove_pool_item_request,
            ))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadShutdownRequest(1)))
            .unwrap();

        target.message_loop();

        // there should be 2 response message on the response channel; throw the first one from the init away
        response_receive.recv().unwrap();

        // there should be one get state response message on the response channel
        let remove_pool_item_response: RemovePoolItemResponse =
            response_receive.recv().unwrap().into();

        assert_eq!(id, remove_pool_item_response.id());
        assert!(target.pool_item_map.is_empty());
    }

    #[test]
    fn send_remove_element_with_id_2_expected_element_removed_from_map_set() {
        let id = 2;
        let init_request = RandomsAddRequest(id);

        let remove_pool_item_request = RemovePoolItemRequest(id);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();

        // send the remove request
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                remove_pool_item_request,
            ))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadShutdownRequest(1)))
            .unwrap();

        target.message_loop();

        // there should be 2 response message on the response channel; throw the first one from the init away
        response_receive.recv().unwrap();

        // there should be one get state response message on the response channel
        let remove_pool_item_response: RemovePoolItemResponse =
            response_receive.recv().unwrap().into();

        assert_eq!(id, remove_pool_item_response.id());

        assert!(target.pool_item_map.is_empty());
    }

    #[test]
    fn init_id_1_2_thread_shutdown_clears_the_elements_returns_expected_shutdown_threads() {
        let init_request_0 = RandomsAddRequest(1);
        let init_request_1 = RandomsAddRequest(2);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(15, request_receive);

        // send the init requests
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request_0))
            .unwrap();
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request_1))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadShutdownRequest(15)))
            .unwrap();

        target.message_loop();

        // there should be 3 response message on the response channel; throw the first two from the init away
        response_receive.recv().unwrap();
        response_receive.recv().unwrap();

        let thread_shutdown_payload: ThreadShutdownResponse =
            response_receive.recv().unwrap().into();

        // there should be one thread shutdown
        // Randoms pool item "pretends" that it has shutdown a thread pool and returns its id
        // as there are 2 pool items is is non-deterministic which one will get called
        assert!(
            thread_shutdown_payload
                == ThreadShutdownResponse::new(15, vec![ThreadShutdownResponse::new(1, vec![])])
                || thread_shutdown_payload
                    == ThreadShutdownResponse::new(
                        15,
                        vec![ThreadShutdownResponse::new(2, vec![])]
                    )
        );
        assert!(target.pool_item_map.is_empty());
    }

    #[test]
    fn init_id_101_102_thread_shutdown_clears_the_elements_returns_expected_shutdown_threads() {
        let init_request_0 = RandomsAddRequest(101);
        let init_request_1 = RandomsAddRequest(102);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(5, request_receive);

        // send the init requests
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request_0))
            .unwrap();
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request_1))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadShutdownRequest(5)))
            .unwrap();

        target.message_loop();

        // there should be 3 response message on the response channel; throw the first two from the init away
        response_receive.recv().unwrap();
        response_receive.recv().unwrap();

        // there should be one thread shutdown
        let thread_shutdown_response: ThreadShutdownResponse =
            response_receive.recv().unwrap().into();
        // Randoms pool item "pretends" that it has shutdown a thread pool with an id equal to its id
        // as there are 2 pool items it is not deterministic which one will have shutdown called
        assert!(
            thread_shutdown_response
                == ThreadShutdownResponse::new(5, vec![ThreadShutdownResponse::new(101, vec![])])
                || thread_shutdown_response
                    == ThreadShutdownResponse::new(
                        5,
                        vec![ThreadShutdownResponse::new(102, vec![])]
                    )
        );
        assert!(target.pool_item_map.is_empty());
    }

    #[test]
    fn init_id_101_send_get_state_message_to_element_retrieved_expected_response() {
        let id = 101;
        let init_request = RandomsAddRequest(id);
        let get_state_request = SumRequest(id);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();

        // send the get state request
        request_send
            .send(SenderCouplet::new(response_send.clone(), get_state_request))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadShutdownRequest(1)))
            .unwrap();

        target.message_loop();

        // there should be 2 response message on the response channel; throw the first one from the init away
        response_receive.recv().unwrap();

        let response: SumResponse = response_receive.recv().unwrap().into();
        // there should be one get state response message on the response channel
        assert_eq!(id, response.id);
        assert!(response.sum > 0);
    }

    #[test]
    fn send_init_id_2_expected_element_added_to_map_set() {
        let id = 2;
        let init_request = RandomsAddRequest(id);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(3, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();
        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(response_send, ThreadAbortRequest(3)))
            .unwrap();

        target.message_loop();

        let response: AddResponse = response_receive.recv().unwrap().into();

        assert_eq!(2, response.id());
        assert_eq!(1, target.pool_item_map.len());
        assert_eq!(2, target.pool_item_map.get(&id).unwrap().id);
    }

    #[test]
    fn send_init_id_1_expected_element_added_to_map() {
        let id = 1;
        let init_request = RandomsAddRequest(id);

        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();
        // send the abort message so that the message loop exits and keeps the state
        request_send
            .send(SenderCouplet::new(response_send, ThreadAbortRequest(1)))
            .unwrap();

        target.message_loop();

        let response: AddResponse = response_receive.recv().unwrap().into();

        assert_eq!(1, response.id());
        assert_eq!(1, target.pool_item_map.len());
        assert_eq!(1, target.pool_item_map.get(&id).unwrap().id);
    }

    #[test]
    fn echo_message_responds_as_expected() {
        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                // target id 2 will get processed by thread 1 as there is only one thread
                ThreadEchoRequest::new(2, "ping".to_string()),
            ))
            .unwrap();
        request_send
            .send(SenderCouplet::new(response_send, ThreadShutdownRequest(1)))
            .unwrap();

        target.message_loop();

        let thread_echo_response: ThreadEchoResponse = response_receive.recv().unwrap().into();

        assert_eq!("ping".to_string(), thread_echo_response.message());
        assert_eq!(2, thread_echo_response.thread_id());
        assert_eq!(1, thread_echo_response.responding_thread_id());
    }

    #[test]
    fn id_2_receives_abort_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(2, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadAbortRequest(2),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't abort/shutdown

        // there should be one thread shutdown response on the response channel
        let thread_abort_response: ThreadAbortResponse = response_receive.recv().unwrap().into();

        assert_eq!(2, thread_abort_response.thread_id());
    }

    #[test]
    fn id_1_receives_abort_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadAbortRequest(1),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't abort/shutdown

        // there should be one thread abort response on the response channel
        let thread_abort_response: ThreadAbortResponse = response_receive.recv().unwrap().into();

        assert_eq!(1, thread_abort_response.thread_id());
    }

    #[test]
    fn id_2_receives_shutdown_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(2, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadShutdownRequest(2),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't shutdown

        // there should be one thread shutdown response on the response channel
        let thread_shutdown_payload: ThreadShutdownResponse =
            response_receive.recv().unwrap().into();

        assert_eq!(2, thread_shutdown_payload.thread_id());
        assert_eq!(
            &Vec::<ThreadShutdownResponse>::default(),
            thread_shutdown_payload.children()
        )
    }

    #[test]
    fn id_1_receives_shutdown_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadRequestResponse<Randoms>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadShutdownRequest(1),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't shutdown

        // there should be one thread shutdown response on the response channel
        let thread_shutdown_payload: ThreadShutdownResponse =
            response_receive.recv().unwrap().into();

        assert_eq!(1, thread_shutdown_payload.thread_id());
        assert_eq!(
            &Vec::<ThreadShutdownResponse>::default(),
            thread_shutdown_payload.children()
        )
    }
}
