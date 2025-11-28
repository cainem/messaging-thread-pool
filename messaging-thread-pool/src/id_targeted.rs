use std::fmt::Debug;

/// A trait for types that have an ID used for routing within the thread pool.
///
/// This trait is fundamental to how `messaging_thread_pool` works. The ID is used to:
/// - Route requests to the correct thread (`id % thread_count`)
/// - Identify which pool item should process a message
/// - Associate responses with their original requests
///
/// # Implementation Requirements
///
/// - IDs must be unique within a thread pool for pool items
/// - IDs should be stable (not change during the object's lifetime)
/// - The `id()` method should be cheap to call (typically just returning a field)
///
/// # Example
///
/// ```rust
/// use messaging_thread_pool::IdTargeted;
///
/// #[derive(Debug)]
/// struct MyItem {
///     id: u64,
///     data: String,
/// }
///
/// impl IdTargeted for MyItem {
///     fn id(&self) -> u64 {
///         self.id
///     }
/// }
///
/// // Request types also implement IdTargeted
/// #[derive(Debug)]
/// struct MyRequest(u64, String);
///
/// impl IdTargeted for MyRequest {
///     fn id(&self) -> u64 {
///         self.0  // First field is the target ID
///     }
/// }
/// ```
///
/// # Note on Generated Types
///
/// When using the `#[pool_item]` macro, `IdTargeted` is automatically implemented
/// for all generated request and response types. You only need to implement it
/// manually for:
/// - Your pool item struct
/// - Custom initialization request types (when using `Init = "..."`)
pub trait IdTargeted: Debug {
    /// Returns the ID used for routing this type within the thread pool.
    fn id(&self) -> u64;
}

impl IdTargeted for u64 {
    fn id(&self) -> u64 {
        *self
    }
}
