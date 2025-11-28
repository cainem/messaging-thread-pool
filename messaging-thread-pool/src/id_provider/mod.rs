//! # ID Provider Utilities
//!
//! This module provides traits and implementations for generating unique IDs
//! for pool items.
//!
//! ## When You Need an ID Provider
//!
//! For simple cases where you create pool items with explicit IDs, you don't need
//! an ID provider:
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, samples::*};
//!
//! let pool = ThreadPool::<Randoms>::new(4);
//!
//! // Manually assign IDs
//! pool.send_and_receive((0..100u64).map(RandomsAddRequest)).unwrap();
//! ```
//!
//! ID providers become useful when:
//! - Multiple components create pool items and need unique IDs
//! - IDs need to be generated dynamically at runtime
//! - Multiple thread pools share a single namespace of IDs
//!
//! ## Example: Shared ID Provider
//!
//! ```rust
//! use std::sync::Arc;
//! use messaging_thread_pool::id_provider::{IdProvider, id_provider_mutex::IdProviderMutex};
//!
//! // Create a shared ID provider
//! let id_provider = Arc::new(IdProviderMutex::new(0));
//!
//! // Multiple components can safely get unique IDs
//! let id1 = id_provider.next_id(); // 0
//! let id2 = id_provider.next_id(); // 1
//!
//! // Or clone the Arc to share across threads
//! let provider_clone = Arc::clone(&id_provider);
//! ```
//!
//! ## Implementations
//!
//! - [`id_provider_mutex::IdProviderMutex`] - Thread-safe provider using a `Mutex`
//! - [`id_provider_static::IdProviderStatic`] - Static/global provider (for testing)

use std::fmt::Debug;

pub mod id_provider_mutex;
pub mod id_provider_static;

/// A trait for generating unique IDs for pool items.
///
/// Implementations must be thread-safe (`Send + Sync`) since ID providers
/// are often shared across multiple threads.
///
/// # Example Implementation
///
/// ```rust
/// use std::sync::atomic::{AtomicU64, Ordering};
/// use messaging_thread_pool::id_provider::IdProvider;
///
/// #[derive(Debug)]
/// struct AtomicIdProvider {
///     counter: AtomicU64,
/// }
///
/// impl IdProvider for AtomicIdProvider {
///     fn next_id(&self) -> u64 {
///         self.counter.fetch_add(1, Ordering::SeqCst)
///     }
///
///     fn peek_next_id(&self) -> u64 {
///         self.counter.load(Ordering::SeqCst)
///     }
/// }
/// ```
pub trait IdProvider: Debug + Send + Sync {
    /// Returns the next ID without advancing the counter.
    ///
    /// Useful for debugging or logging what the next ID will be.
    fn peek_next_id(&self) -> u64;

    /// Returns the next ID and advances the counter.
    ///
    /// Each call returns a unique ID (assuming single-threaded access
    /// or proper synchronization in the implementation).
    fn next_id(&self) -> u64;
}
