use messaging_thread_pool_macros::pool_item;

// Allow the macro to refer to the crate by name
use crate as messaging_thread_pool;

/// A simple chat room that manages a history of messages.
///
/// This sample demonstrates the most basic `#[pool_item]` usage with:
/// - Mutable state (the `history` vector)
/// - Multiple message types (`post`, `get_history`)
/// - Minimal boilerplate
///
/// # Generated Types
///
/// The `#[pool_item]` macro generates:
/// - `ChatRoomInit(u64)` - Create a new chat room with the given ID
/// - `ChatRoomApi` - Enum of message types
/// - `PostRequest(u64, String, String)` - Post a message (id, user, text)
/// - `PostResponse { id, result }` - Response with message index
/// - `GetHistoryRequest(u64)` - Request message history
/// - `GetHistoryResponse { id, result }` - Response with history vector
///
/// # Example
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::*};
///
/// let pool = ThreadPool::<ChatRoom>::new(2);
///
/// // Create room with ID 1
/// pool.send_and_receive_once(ChatRoomInit(1)).unwrap();
///
/// // Post messages
/// pool.send_and_receive_once(PostRequest(1, "Alice".into(), "Hello!".into())).unwrap();
/// pool.send_and_receive_once(PostRequest(1, "Bob".into(), "Hi Alice!".into())).unwrap();
///
/// // Get history
/// let history = pool.send_and_receive_once(GetHistoryRequest(1)).unwrap();
/// assert_eq!(history.result.len(), 2);
/// assert_eq!(history.result[0], "Alice: Hello!");
/// ```
///
/// # Note
///
/// This is the simplest example in the samples. For shared state patterns with
/// `Rc<RefCell<T>>`, see [`UserSession`](super::UserSession).
#[derive(Debug)]
pub struct ChatRoom {
    #[allow(dead_code)]
    id: u64,
    /// The history of messages in this room
    pub history: Vec<String>,
}

impl ChatRoom {
    /// Called by the thread pool when a room with this ID is first requested
    pub fn new(id: u64) -> Self {
        Self {
            id,
            history: Vec::new(),
        }
    }
}

#[pool_item]
impl ChatRoom {
    /// Post a message to the room.
    ///
    /// Returns the index of the posted message (0-based).
    #[messaging(PostRequest, PostResponse)]
    pub fn post(&mut self, user: String, text: String) -> usize {
        let entry = format!("{}: {}", user, text);
        self.history.push(entry);
        self.history.len() - 1
    }

    /// Retrieve the entire message history.
    ///
    /// Returns a clone of all messages posted to this room.
    #[messaging(GetHistoryRequest, GetHistoryResponse)]
    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
}
