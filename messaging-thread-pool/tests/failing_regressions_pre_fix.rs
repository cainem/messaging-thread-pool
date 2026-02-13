use messaging_thread_pool::{
    SenderAndReceiver, SenderAndReceiverMock, ThreadPool,
    id_provider::{IdProvider, id_provider_mutex::IdProviderMutex},
    samples::{MeanRequest, Randoms, RandomsAddRequest, SumRequest},
};

#[test]
fn given_id_provider_mutex_when_cloned_then_clone_succeeds_and_preserves_next_id() {
    let provider = IdProviderMutex::new(7);

    let cloned = provider.clone();

    assert_eq!(provider.peek_next_id(), cloned.peek_next_id());
}

#[test]
fn given_shutdown_thread_pool_when_sending_then_returns_error_instead_of_panicking() {
    let pool = ThreadPool::<Randoms>::new(1);

    let _ = pool.shutdown();

    let result = pool.send_and_receive_once(RandomsAddRequest(0));

    assert!(
        result.is_err(),
        "sending after shutdown should return an error"
    );
}

#[test]
fn given_sender_and_receiver_mock_with_no_responses_when_send_and_receive_one_then_no_panic() {
    let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = mock.send_and_receive_one(MeanRequest(123));
    }));

    assert!(
        result.is_ok(),
        "send_and_receive_one should not panic when a response is missing"
    );
}

#[test]
fn given_missing_pool_item_id_when_message_sent_then_no_panic() {
    let pool = ThreadPool::<Randoms>::new(1);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = pool.send_and_receive_once(SumRequest(999));
    }));

    assert!(
        result.is_ok(),
        "sending to a missing id should be handled without panic"
    );
}
