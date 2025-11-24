use crate::pool_item::PoolItem;
use messaging_thread_pool_macros::pool_item;

// Allow the macro to refer to the crate by name
use crate as messaging_thread_pool;

/// A ChatRoom is a stateful entity that manages a list of messages.
/// It is managed by the thread pool, so we don't need internal mutexes.
#[derive(Debug)]
pub struct ChatRoom {
    id: u64,
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
    /// Returns the index of the message.
    #[messaging(PostRequest, PostResponse)]
    pub fn post(&mut self, user: String, text: String) -> usize {
        let entry = format!("{}: {}", user, text);
        self.history.push(entry);
        self.history.len() - 1
    }

    /// Retrieve the entire message history.
    #[messaging(GetHistoryRequest, GetHistoryResponse)]
    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
}
