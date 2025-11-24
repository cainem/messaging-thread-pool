use messaging_thread_pool::{samples::*, ThreadPool};

#[test]
pub fn example_chat_room() {
    // Create a thread pool with 2 threads
    let thread_pool = ThreadPool::<ChatRoom>::new(2);

    // Room IDs
    let room_general = 1;
    let room_tech = 2;

    // Create the rooms
    thread_pool
        .send_and_receive(vec![
            ChatRoomInit(room_general),
            ChatRoomInit(room_tech),
        ].into_iter())
        .expect("creation requests")
        .for_each(|_| {});

    // Post messages to "General" (Room 1)
    // The pool will route these to the thread responsible for Room 1
    thread_pool
        .send_and_receive(vec![
            PostRequest(room_general, "Alice".to_string(), "Hello everyone!".to_string()),
            PostRequest(room_general, "Bob".to_string(), "Hi Alice!".to_string()),
        ].into_iter())
        .expect("messages to send")
        .for_each(|response| {
            // Responses are the index of the message
            assert!(response.result < 100);
        });

    // Post messages to "Tech" (Room 2)
    thread_pool
        .send_and_receive(vec![
            PostRequest(room_tech, "Charlie".to_string(), "Rust is cool".to_string()),
        ].into_iter())
        .expect("messages to send")
        .for_each(|_| {});

    // Retrieve history from General
    let history_response = thread_pool
        .send_and_receive(vec![GetHistoryRequest(room_general)].into_iter())
        .expect("request to send")
        .next()
        .expect("response");

    let history = history_response.result;
    
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "Alice: Hello everyone!");
    assert_eq!(history[1], "Bob: Hi Alice!");

    // Retrieve history from Tech
    let history_response = thread_pool
        .send_and_receive(vec![GetHistoryRequest(room_tech)].into_iter())
        .expect("request to send")
        .next()
        .expect("response");

    let history = history_response.result;
    
    assert_eq!(history.len(), 1);
    assert_eq!(history[0], "Charlie: Rust is cool");
}
