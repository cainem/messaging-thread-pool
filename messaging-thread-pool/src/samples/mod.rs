//! # Sample Pool Item Implementations
//!
//! This module contains example `PoolItem` implementations that demonstrate
//! different patterns and use cases for the library.
//!
//! ## Samples Overview
//!
//! | Sample | Complexity | Demonstrates |
//! |--------|------------|--------------|
//! | [`UserSession`] | Beginner | `Rc<RefCell<T>>`, helper structs, basic `#[pool_item]` usage |
//! | [`ChatRoom`] | Beginner | Simple state management, minimal boilerplate |
//! | [`Randoms`] | Intermediate | Shutdown hooks, benchmarking patterns |
//! | [`RandomsBatch`] | Advanced | Generics, nested thread pools, custom Init types |
//!
//! ## Recommended Learning Path
//!
//! ### 1. Start with `UserSession`
//!
//! [`UserSession`] is the canonical example showing the main advantage of this library:
//! using `Rc` and `RefCell` for shared state without locks.
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, samples::*};
//!
//! let pool = ThreadPool::<UserSession>::new(2);
//!
//! // Create a session
//! pool.send_and_receive_once(UserSessionInit(1)).unwrap();
//!
//! // Log actions
//! pool.send_and_receive_once(LogActionRequest(1, "Login".to_string())).unwrap();
//!
//! // Get the log
//! let log: Vec<String> = pool
//!     .send_and_receive_once(GetLogRequest(1))
//!     .unwrap()
//!     .result;
//! ```
//!
//! ### 2. See `ChatRoom` for Minimal Boilerplate
//!
//! [`ChatRoom`] shows the simplest possible `#[pool_item]` usage with mutable state:
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, samples::*};
//!
//! let pool = ThreadPool::<ChatRoom>::new(2);
//!
//! pool.send_and_receive_once(ChatRoomInit(1)).unwrap();
//! pool.send_and_receive_once(PostRequest(1, "Alice".into(), "Hello!".into())).unwrap();
//!
//! let history = pool.send_and_receive_once(GetHistoryRequest(1)).unwrap();
//! assert_eq!(history.result, vec!["Alice: Hello!"]);
//! ```
//!
//! ### 3. See `Randoms` for Shutdown Hooks
//!
//! [`Randoms`] demonstrates the `Shutdown` parameter for cleanup on pool termination:
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, samples::*};
//!
//! let pool = ThreadPool::<Randoms>::new(2);
//! pool.send_and_receive_once(RandomsAddRequest(1)).unwrap();
//!
//! // Shutdown returns responses from each item
//! let shutdown_responses = pool.shutdown();
//! ```
//!
//! ### 4. See `RandomsBatch` for Advanced Patterns
//!
//! [`RandomsBatch`] shows complex patterns including:
//! - Generic pool items
//! - Custom initialization types (`Init = "RandomsBatchAddRequest<P>"`)
//! - Nested thread pools (a `RandomsBatch` contains references to another pool)
//! - Mocking nested pools for testing
//!
//! ## Using Samples in Tests
//!
//! These samples are re-exported for use in your tests and benchmarks:
//!
//! ```rust
//! use messaging_thread_pool::samples::*;
//! ```
//!
//! See the integration tests in `tests/` for more complete examples of each pattern.

mod chat_room;
mod randoms;
mod randoms_batch;
mod user_session;

// re-export
pub use chat_room::*;
pub use randoms::*;
pub use randoms_batch::*;
pub use user_session::*;
