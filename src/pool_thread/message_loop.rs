use tracing::{event, instrument, Level};

use crate::{
    element::Element, id_targeted::IdTargeted, thread_request::ThreadRequest,
    thread_response::ThreadResponse, thread_shutdown_response::ThreadShutdownResponse,
};

use super::PoolThread;

impl<E> PoolThread<E>
where
    E: Element,
{
    /// This function forms the "message loop" of the [`PoolThread`]
    ///
    /// This is an infinite loop that pulls requests off of the [`PoolThread`]s receive channel
    /// This function will be running in the context of its own dedicated thread
    ///
    /// Messages are split into 2 types, element requests and non-element requests
    /// Element requests are forwarded to the appropriate target element (that is contained in a local
    /// hashmap)
    /// The message can be targeted by virtue of the fact they all contain a key of the target that
    /// the message is intended for.
    ///
    /// Non-element requests are handled within this loop and are used to control the thread pool
    /// These include ThreadShutdown, ThreadAbort and ElementRemove
    ///
    /// ThreadShutdown and ThreadAAbort cause the message loop to exit and thus end the thread.
    #[instrument(skip(self))]
    pub fn message_loop(&mut self) {
        // will loop until the queue is empty and the sender is dropped
        while let Ok(sender_couplet) = self.pool_thread_receiver.recv() {
            event!(
                Level::TRACE,
                "thread {:?} receiving request for {:?}",
                self.id,
                sender_couplet.get_thread_request(),
            );

            let response = match sender_couplet.get_thread_request() {
                ThreadRequest::ElementRequest(request) => {
                    let id = request.get_id();

                    // optionally add tracing at what is effectively the element level this allows for the ability
                    // to do sampling.
                    // so for example every 1000th id could be traced
                    let tracing_guards = E::add_element_request_tracing(id);

                    let response = ThreadResponse::ElementResponse(
                        if let Some(targeted) = self.element_hash_map.get_mut(&id) {
                            // if the id already exists then it must be a message that needs processing that needs processing against an
                            // existing element
                            targeted.process_message_checked(request)
                        } else {
                            // element did exist therefore it can only be a request to create a new element
                            match E::new_element(request) {
                                // return a new element and a response
                                // the new element needs adding to the hash map and the response needs returning
                                (Some(new_element), response) => {
                                    event!(
                                        Level::DEBUG,
                                        "Inserting a new {:?} into the threads hash map",
                                        new_element.name()
                                    );
                                    self.element_hash_map.insert(id, new_element);
                                    response
                                }
                                // unable to create, still return the response
                                (None, response) => response,
                            }
                        },
                    );

                    // drop the tracing
                    drop(tracing_guards);

                    response
                }
                ThreadRequest::RemoveElement(id) => {
                    debug_assert!(
                        self.element_hash_map.remove(id).is_some(),
                        "insist that the element exists"
                    );
                    ThreadResponse::RemoveElement(*id)
                }
                ThreadRequest::ThreadShutdown(id) => {
                    debug_assert_eq!(
                        self.id as u64, *id,
                        "this messages should have targeted this thread"
                    );
                    // this call to shutdown the child threads and consequently empty the internal hash map
                    // is how thread shutdown differs from thread abort. Abort just exist the loop and leaves the
                    // state in place
                    let children = self.shutdown_child_pool();
                    sender_couplet
                        .get_return_to()
                        .send(ThreadResponse::ThreadShutdown(ThreadShutdownResponse::new(
                            *id, children,
                        )))
                        .expect("the send should always succeed");
                    debug_assert!(
                        self.element_hash_map.is_empty(),
                        "ThreadShutdown should drain all elements"
                    );
                    return;
                }
                ThreadRequest::ThreadAbort(id) => {
                    debug_assert_eq!(
                        self.id as u64, *id,
                        "this messages should have targeted this thread"
                    );
                    sender_couplet
                        .get_return_to()
                        .send(ThreadResponse::ThreadAbort(*id))
                        .expect("the send should always succeed");
                    return;
                }
                ThreadRequest::ThreadEcho(targeted_id, message) => ThreadResponse::ThreadEcho(
                    *targeted_id,
                    self.id as u64,
                    format!("{} [{}]", message, self.id),
                ),
            };

            event!(Level::TRACE, ?response, message = "sending response");

            sender_couplet
                .get_return_to()
                .send(response)
                .expect("the send should always succeed");

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
        thread_request::ThreadRequest, thread_response::ThreadResponse,
        thread_shutdown_response::ThreadShutdownResponse,
    };

    #[test]
    fn send_remove_element_with_id_12_expected_element_removed_from_hash_set() {
        let id = 12;
        let init_request = InitRequest { id };

        let remove_element_request = ThreadRequest::RemoveElement(id);

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
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
                remove_element_request,
            ))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadShutdown(1),
            ))
            .unwrap();

        target.message_loop();

        // there should be 2 response message on the response channel; throw the first one from the init away
        response_receive.recv().unwrap();

        // there should be one get state response message on the response channel
        if let ThreadResponse::RemoveElement(remove_id) = response_receive.recv().unwrap() {
            assert_eq!(id, remove_id);
        } else {
            panic!("not expected");
        }

        assert!(target.element_hash_map.is_empty());
    }

    #[test]
    fn send_remove_element_with_id_2_expected_element_removed_from_hash_set() {
        let id = 2;
        let init_request = InitRequest { id };

        let remove_element_request = ThreadRequest::RemoveElement(id);

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
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
                remove_element_request,
            ))
            .unwrap();

        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadShutdown(1),
            ))
            .unwrap();

        target.message_loop();

        // there should be 2 response message on the response channel; throw the first one from the init away
        response_receive.recv().unwrap();

        // there should be one get state response message on the response channel
        if let ThreadResponse::RemoveElement(remove_id) = response_receive.recv().unwrap() {
            assert_eq!(id, remove_id);
        } else {
            panic!("not expected");
        }

        assert!(target.element_hash_map.is_empty());
    }

    #[test]
    fn init_id_1_2_thread_shutdown_clears_the_elements_returns_expected_shutdown_threads() {
        let init_request_0 = InitRequest { id: 1 };
        let init_request_1 = InitRequest { id: 2 };

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
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
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadShutdown(15),
            ))
            .unwrap();

        target.message_loop();

        // there should be 3 response message on the response channel; throw the first two from the init away
        response_receive.recv().unwrap();
        response_receive.recv().unwrap();

        // there should be one thread shutdown
        if let ThreadResponse::ThreadShutdown(thread_shutdown_payload) =
            response_receive.recv().unwrap()
        {
            // Randoms element "pretends" that it has shutdown a thread pool and returns its id
            // as there are 2 element is is non-deterministic which one will get called
            assert!(
                thread_shutdown_payload
                    == ThreadShutdownResponse::new(
                        15,
                        vec![ThreadShutdownResponse::new(1, vec![])]
                    )
                    || thread_shutdown_payload
                        == ThreadShutdownResponse::new(
                            15,
                            vec![ThreadShutdownResponse::new(2, vec![])]
                        )
            );
            assert!(target.element_hash_map.is_empty());
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn init_id_101_102_thread_shutdown_clears_the_elements_returns_expected_shutdown_threads() {
        let init_request_0 = InitRequest { id: 101 };
        let init_request_1 = InitRequest { id: 102 };

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
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
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadShutdown(5),
            ))
            .unwrap();

        target.message_loop();

        // there should be 3 response message on the response channel; throw the first two from the init away
        response_receive.recv().unwrap();
        response_receive.recv().unwrap();

        // there should be one thread shutdown
        if let ThreadResponse::ThreadShutdown(thread_shutdown_payload) =
            response_receive.recv().unwrap()
        {
            // Randoms element "pretends" that it has shutdown a thread pool with an id equal to its id
            // as there are 2 elements it is not deterministic which one will have shutdown called
            assert!(
                thread_shutdown_payload
                    == ThreadShutdownResponse::new(
                        5,
                        vec![ThreadShutdownResponse::new(101, vec![])]
                    )
                    || thread_shutdown_payload
                        == ThreadShutdownResponse::new(
                            5,
                            vec![ThreadShutdownResponse::new(102, vec![])]
                        )
            );
            assert!(target.element_hash_map.is_empty());
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn init_id_101_send_get_state_message_to_element_retrieved_expected_response() {
        let id = 101;
        let init_request = InitRequest { id };
        let get_state_request = SumRequest { id: 101 };

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
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
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadShutdown(1),
            ))
            .unwrap();

        target.message_loop();

        // there should be 2 response message on the response channel; throw the first one from the init away
        response_receive.recv().unwrap();

        // there should be one get state response message on the response channel
        if let ThreadResponse::ElementResponse(RandomsResponse::Sum(response)) =
            response_receive.recv().unwrap()
        {
            assert_eq!(id, response.id);
            assert!(response.sum > 0);
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn send_init_id_2_expected_element_added_to_hash_set() {
        let id = 2;
        let init_request = InitRequest { id };

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(3, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();
        // send the shutdown message so that the message loop exits
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadAbort(3),
            ))
            .unwrap();

        target.message_loop();

        // there should be one init response message on the response channel
        if let ThreadResponse::ElementResponse(RandomsResponse::Init(response)) =
            response_receive.recv().unwrap()
        {
            assert_eq!(2, response.id)
        } else {
            panic!("not expected");
        }

        assert_eq!(1, target.element_hash_map.len());
        assert_eq!(2, target.element_hash_map.get(&id).unwrap().id);
    }

    #[test]
    fn send_init_id_1_expected_element_added_to_hash_set() {
        let id = 1;
        let init_request = InitRequest { id };

        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        // send the init request
        request_send
            .send(SenderCouplet::new(response_send.clone(), init_request))
            .unwrap();
        // send the abort message so that the message loop exits and keeps the state
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadAbort(1),
            ))
            .unwrap();

        target.message_loop();

        // there should be one init response message on the response channel
        if let RandomsResponse::Init(response) = response_receive.recv().unwrap().into() {
            assert_eq!(1, response.id)
        } else {
            panic!("not expected");
        }

        assert_eq!(1, target.element_hash_map.len());
        assert_eq!(1, target.element_hash_map.get(&id).unwrap().id);
    }

    #[test]
    fn echo_message_responds_as_expected() {
        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                // target id 2 will get processed by thread 1 as there is only one thread
                ThreadRequest::ThreadEcho(2, "ping".to_string()),
            ))
            .unwrap();
        request_send
            .send(SenderCouplet::new(
                response_send.clone(),
                ThreadRequest::ThreadShutdown(1),
            ))
            .unwrap();

        target.message_loop();

        // there should be one echo message on the response channel
        if let ThreadResponse::ThreadEcho(targeted_id, actual_thread_id, value) =
            response_receive.recv().unwrap()
        {
            assert_eq!("ping [1]".to_string(), value);
            assert_eq!(2, targeted_id);
            // processed by thread 1
            assert_eq!(1, actual_thread_id);
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn id_2_receives_abort_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(2, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadRequest::<RandomsRequest>::ThreadAbort(2),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't abort/shutdown

        // there should be one thread shutdown response on the response channel
        if let ThreadResponse::ThreadAbort(id) = response_receive.recv().unwrap() {
            assert_eq!(2, id);
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn id_1_receives_abort_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadRequest::<RandomsRequest>::ThreadAbort(1),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't abort/shutdown

        // there should be one thread shutdown response on the response channel
        if let ThreadResponse::ThreadAbort(id) = response_receive.recv().unwrap() {
            assert_eq!(1, id);
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn id_2_receives_shutdown_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(2, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadRequest::<RandomsRequest>::ThreadShutdown(2),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't shutdown

        // there should be one thread shutdown response on the response channel
        if let ThreadResponse::ThreadShutdown(thread_shutdown_payload) =
            response_receive.recv().unwrap()
        {
            assert_eq!(2, thread_shutdown_payload.id());
        } else {
            panic!("not expected");
        }
    }

    #[test]
    fn id_1_receives_shutdown_message_exits_loop() {
        let (response_send, response_receive) = unbounded::<ThreadResponse<RandomsResponse>>();
        let (request_send, request_receive) = unbounded::<SenderCouplet<Randoms>>();

        let mut target = PoolThread::new(1, request_receive);

        request_send
            .send(SenderCouplet::<Randoms>::new(
                response_send,
                ThreadRequest::<RandomsRequest>::ThreadShutdown(1),
            ))
            .unwrap();

        target.message_loop();

        // the test would never return if the loop didn't shutdown

        // there should be one thread shutdown response on the response channel
        if let ThreadResponse::ThreadShutdown(thread_shutdown_payload) =
            response_receive.recv().unwrap()
        {
            assert_eq!(1, thread_shutdown_payload.id());
            assert_eq!(
                Vec::<ThreadShutdownResponse>::default(),
                thread_shutdown_payload.children()
            )
        } else {
            panic!("not expected");
        }
    }
}
