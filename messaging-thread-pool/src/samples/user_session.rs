//! # UserSession Example
//!
//! This module provides a canonical example of using the `#[pool_item]` macro
//! to create a pool item that leverages the key advantage of this library:
//! **using `Rc` and `RefCell` for shared state without locks**.
//!
//! ## The Problem This Solves
//!
//! In traditional thread pools (like `rayon`), any shared state must be wrapped
//! in `Arc<Mutex<...>>` because work can migrate between threads. This adds:
//! - Lock contention overhead
//! - Complexity in reasoning about concurrent access
//! - Risk of deadlocks
//!
//! With `messaging_thread_pool`, each pool item is **pinned to a single thread**
//! for its entire lifetime. This means you can safely use:
//! - `Rc<T>` instead of `Arc<T>`
//! - `RefCell<T>` instead of `Mutex<T>`
//! - Raw pointers and FFI resources that aren't `Send`/`Sync`
//!
//! ## Example Structure
//!
//! This example demonstrates:
//! 1. A `UserSession` pool item that tracks user actions
//! 2. A `HistoryTracker` helper struct that shares access to the session's log
//! 3. Both structs share the same `Vec<String>` via `Rc<RefCell<...>>`
//!
//! ## Usage
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, samples::*};
//!
//! // Create a thread pool with 2 threads
//! let thread_pool = ThreadPool::<UserSession>::new(2);
//!
//! // Create a session with ID 1
//! thread_pool
//!     .send_and_receive(vec![UserSessionInit(1)].into_iter())
//!     .expect("session creation")
//!     .for_each(|_| {});
//!
//! // Log some actions - these are processed sequentially by the thread owning Session 1
//! let counts: Vec<usize> = thread_pool
//!     .send_and_receive(vec![
//!         LogActionRequest(1, "Login".to_string()),
//!         LogActionRequest(1, "ViewProfile".to_string()),
//!         LogActionRequest(1, "Logout".to_string()),
//!     ].into_iter())
//!     .expect("actions")
//!     .map(|resp| resp.result)
//!     .collect();
//!
//! assert_eq!(counts, vec![1, 2, 3]);
//!
//! // Retrieve the full log
//! let log: Vec<String> = thread_pool
//!     .send_and_receive(vec![GetLogRequest(1)].into_iter())
//!     .expect("get log")
//!     .next()
//!     .unwrap()
//!     .result;
//!
//! assert_eq!(log[0], "Action: Login");
//! assert_eq!(log[1], "Action: ViewProfile");
//! assert_eq!(log[2], "Action: Logout");
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use crate::IdTargeted;
use crate::pool_item;

/// A helper struct that needs access to the session's data.
///
/// In a standard thread pool, this would likely need `Arc<Mutex<Vec<String>>>`.
/// Here, we can use `Rc<RefCell<...>>` because `UserSession` never leaves its thread.
///
/// This pattern is useful when you have:
/// - Helper objects that need to modify shared state
/// - Multiple components within a pool item that need access to the same data
/// - Complex internal structures that would be painful to wrap in `Arc<Mutex<...>>`
#[derive(Debug, Clone)]
pub struct HistoryTracker {
    /// Shared access to the history log - no `Arc`, no `Mutex`!
    log: Rc<RefCell<Vec<String>>>,
}

impl HistoryTracker {
    /// Add an entry to the shared log.
    ///
    /// Note: No locks needed! Just `borrow_mut()`.
    pub fn add_entry(&self, entry: String) {
        self.log.borrow_mut().push(entry);
    }
}

/// A user session that tracks actions performed by a user.
///
/// This is the main pool item. It demonstrates:
/// - Owning non-`Send`/`Sync` data (`Rc<RefCell<...>>`)
/// - Sharing data with helper structs (`HistoryTracker`)
/// - Sequential message processing (messages to the same session are never concurrent)
///
/// ## Generated Types
///
/// The `#[pool_item]` macro on the impl block generates:
/// - `UserSessionInit(u64)` - Request to create a new session
/// - `UserSessionApi` - Enum of all message types
/// - `LogActionRequest(u64, String)` / `LogActionResponse` - For the `log_action` method
/// - `GetLogRequest(u64)` / `GetLogResponse` - For the `get_log` method
#[derive(Debug)]
pub struct UserSession {
    id: u64,
    /// We hold the data
    log: Rc<RefCell<Vec<String>>>,
    /// Our helper also holds a reference to the SAME data
    tracker: HistoryTracker,
}

impl IdTargeted for UserSession {
    fn id(&self) -> u64 {
        self.id
    }
}

/// The `#[pool_item]` macro transforms this impl block to:
/// 1. Generate the `PoolItem` trait implementation
/// 2. Generate request/response structs for each `#[messaging(...)]` method
/// 3. Generate the `UserSessionApi` enum containing all message variants
/// 4. Generate the `UserSessionInit` struct for creating new sessions
#[pool_item]
impl UserSession {
    /// Creates a new UserSession.
    ///
    /// This method is called by the thread pool when a `UserSessionInit` request
    /// is received. The session is created on the thread determined by `id % thread_count`.
    ///
    /// Note how we create an `Rc<RefCell<...>>` and clone it to share with the helper.
    /// This would be impossible in a traditional thread pool where the item might
    /// move between threads.
    pub fn new(id: u64) -> Self {
        let log = Rc::new(RefCell::new(Vec::new()));
        let tracker = HistoryTracker { log: log.clone() };

        Self { id, log, tracker }
    }

    /// Log a user action and return the total number of logged actions.
    ///
    /// The `#[messaging(LogActionRequest, LogActionResponse)]` attribute tells the macro to:
    /// - Generate `LogActionRequest(u64, String)` - the id and the `action` parameter
    /// - Generate `LogActionResponse { id: u64, result: usize }` - the id and return value
    /// - Add a variant to `UserSessionApi` for this request/response pair
    ///
    /// Messages to the same session (same ID) are processed sequentially,
    /// so there's no risk of concurrent modification of the log.
    #[messaging(LogActionRequest, LogActionResponse)]
    pub fn log_action(&self, action: String) -> usize {
        // We use the helper to modify the state
        self.tracker.add_entry(format!("Action: {}", action));

        // We can read the state directly
        self.log.borrow().len()
    }

    /// Retrieve the entire action history.
    ///
    /// Returns a clone of the log because the original `Vec` can't leave the thread
    /// (it's behind an `Rc`).
    #[messaging(GetLogRequest, GetLogResponse)]
    pub fn get_log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    use super::*;

    #[test]
    fn given_user_session_when_logging_actions_then_count_increments() {
        let thread_pool = ThreadPool::<UserSession>::new(2);

        // Create session
        thread_pool
            .send_and_receive(vec![UserSessionInit(1)].into_iter())
            .expect("session creation")
            .for_each(|_| {});

        // Log actions and verify counts
        let counts: Vec<usize> = thread_pool
            .send_and_receive(
                vec![
                    LogActionRequest(1, "Login".to_string()),
                    LogActionRequest(1, "ViewProfile".to_string()),
                    LogActionRequest(1, "Logout".to_string()),
                ]
                .into_iter(),
            )
            .expect("actions")
            .map(|resp| resp.result)
            .collect();

        assert_eq!(counts, vec![1, 2, 3]);
    }

    #[test]
    fn given_user_session_when_getting_log_then_returns_all_entries() {
        let thread_pool = ThreadPool::<UserSession>::new(2);

        // Create session
        thread_pool
            .send_and_receive(vec![UserSessionInit(1)].into_iter())
            .expect("session creation")
            .for_each(|_| {});

        // Log actions
        thread_pool
            .send_and_receive(
                vec![
                    LogActionRequest(1, "Login".to_string()),
                    LogActionRequest(1, "Logout".to_string()),
                ]
                .into_iter(),
            )
            .expect("actions")
            .for_each(|_| {});

        // Get log
        let log = thread_pool
            .send_and_receive(vec![GetLogRequest(1)].into_iter())
            .expect("get log")
            .next()
            .unwrap()
            .result;

        assert_eq!(log.len(), 2);
        assert_eq!(log[0], "Action: Login");
        assert_eq!(log[1], "Action: Logout");
    }

    #[test]
    fn given_multiple_sessions_when_logging_actions_then_each_has_independent_state() {
        let thread_pool = ThreadPool::<UserSession>::new(2);

        // Create two sessions
        thread_pool
            .send_and_receive(vec![UserSessionInit(1), UserSessionInit(2)].into_iter())
            .expect("session creation")
            .for_each(|_| {});

        // Log to session 1
        thread_pool
            .send_and_receive(vec![LogActionRequest(1, "Action1".to_string())].into_iter())
            .expect("action")
            .for_each(|_| {});

        // Log to session 2 (twice)
        thread_pool
            .send_and_receive(
                vec![
                    LogActionRequest(2, "ActionA".to_string()),
                    LogActionRequest(2, "ActionB".to_string()),
                ]
                .into_iter(),
            )
            .expect("actions")
            .for_each(|_| {});

        // Verify independent logs
        let log1 = thread_pool
            .send_and_receive(vec![GetLogRequest(1)].into_iter())
            .expect("get log")
            .next()
            .unwrap()
            .result;

        let log2 = thread_pool
            .send_and_receive(vec![GetLogRequest(2)].into_iter())
            .expect("get log")
            .next()
            .unwrap()
            .result;

        assert_eq!(log1.len(), 1);
        assert_eq!(log2.len(), 2);
    }
}
