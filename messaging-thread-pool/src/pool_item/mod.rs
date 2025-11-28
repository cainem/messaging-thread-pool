pub mod new_pool_item_error;

pub use self::new_pool_item_error::NewPoolItemError;
use crate::{
    id_targeted::IdTargeted, request_with_response::RequestWithResponse, thread_request_response::*,
};
use std::fmt::Debug;
use tracing::{Level, event};

/// The core trait for types managed by a [`ThreadPool`](crate::ThreadPool).
///
/// A `PoolItem` is a stateful object that:
/// - Lives on a single thread for its entire lifetime
/// - Has a unique ID within the pool
/// - Communicates via typed request/response messages
/// - Can own non-`Send`/`Sync` types like `Rc<RefCell<T>>`
///
/// # Using the `#[pool_item]` Macro (Recommended)
///
/// The easiest way to implement `PoolItem` is with the `#[pool_item]` attribute macro,
/// which generates all the boilerplate automatically:
///
/// ```rust
/// use messaging_thread_pool::{IdTargeted, pool_item};
///
/// #[derive(Debug)]
/// pub struct Counter {
///     id: u64,
///     value: i32,
/// }
///
/// impl IdTargeted for Counter {
///     fn id(&self) -> u64 { self.id }
/// }
///
/// #[pool_item]
/// impl Counter {
///     pub fn new(id: u64) -> Self {
///         Self { id, value: 0 }
///     }
///
///     #[messaging(IncrementRequest, IncrementResponse)]
///     pub fn increment(&mut self, amount: i32) -> i32 {
///         self.value += amount;
///         self.value
///     }
/// }
/// ```
///
/// # Associated Types
///
/// - **`Init`**: The request type used to create new instances (e.g., `CounterInit(u64)`)
/// - **`Api`**: An enum of all message types the pool item can handle
/// - **`ThreadStartInfo`**: Optional per-thread state (e.g., for tracing configuration)
///
/// # Lifecycle
///
/// 1. **Creation**: When an `Init` request is received, `new_pool_item` is called
/// 2. **Processing**: Messages are delivered to `process_message` sequentially
/// 3. **Shutdown**: `shutdown_pool` is called when the pool is dropped
///
/// # Thread Affinity
///
/// Pool items are assigned to threads using `id_thread_router(id, thread_count)`.
/// The default implementation is `id % thread_count`. All messages for the same ID
/// go to the same thread, ensuring sequential processing without locks.
///
/// # Manual Implementation
///
/// For advanced use cases or when the macro isn't suitable, you can implement
/// `PoolItem` manually. See the [`samples`](crate::samples) module for examples.
pub trait PoolItem: Debug
where
    Self: Sized,
    Self::Init: Send + IdTargeted + RequestWithResponse<Self, Response = AddResponse>,
    Self::Api: Debug + Send + IdTargeted,
{
    /// The request type for creating new instances of this pool item.
    ///
    /// When the pool receives an `Init` message, it calls `new_pool_item` to
    /// create the instance. The generated `{StructName}Init(u64)` type is typical,
    /// but custom types can be used for complex initialization.
    type Init;

    /// The enum type containing all message variants for this pool item.
    ///
    /// Each variant represents a request/response pair. The `#[pool_item]` macro
    /// generates this as `{StructName}Api` with variants for each `#[messaging]` method.
    type Api;

    /// Per-thread state created when a pool thread starts.
    ///
    /// This is useful for:
    /// - Configuring thread-local tracing/logging
    /// - Setting up thread-local caches or resources
    ///
    /// Set to `()` if not needed (the default).
    type ThreadStartInfo;

    /// Process an incoming message and return a response.
    ///
    /// This method is called for each message sent to the pool item. It typically
    /// contains a `match` statement dispatching to the appropriate handler.
    ///
    /// The `#[pool_item]` macro generates this implementation automatically.
    fn process_message(&mut self, request: Self::Api) -> ThreadRequestResponse<Self>;

    /// Called when a message targets an ID that doesn't exist in the pool.
    ///
    /// The default behavior is to panic. Override this to handle missing IDs gracefully
    /// (e.g., by creating the item on-demand or returning an error response).
    fn id_not_found(request: &Self::Api) -> ThreadRequestResponse<Self> {
        // default behaviour is to panic
        event!(Level::ERROR, "pool item with id {} not found", request.id());
        panic!("pool item with id {} not found", request.id());
    }

    /// Returns the type name for logging purposes.
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Create a new instance from an initialization request.
    ///
    /// Called when the pool receives an `Init` message. Return `Ok(instance)` on success,
    /// or `Err(NewPoolItemError)` if creation fails.
    ///
    /// # Example
    ///
    /// When using the macro, this is generated automatically. For manual implementations:
    ///
    /// ```rust,ignore
    /// fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
    ///     Ok(Self::new(request.id()))
    /// }
    /// ```
    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError>;

    /// Called for each pool item when the pool is shutting down.
    ///
    /// Override this to perform cleanup operations. The returned responses are
    /// collected and can be inspected by the caller.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
    ///     // Perform cleanup...
    ///     vec![ThreadShutdownResponse::new(self.id(), vec![])]
    /// }
    /// ```
    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        Vec::<ThreadShutdownResponse>::default()
    }

    /// Called once when a pool thread starts.
    ///
    /// Returns optional thread-local state that will be passed to
    /// `pool_item_pre_process` and `pool_item_post_process`.
    ///
    /// Primary use case: Configuring tracing/logging for the thread.
    fn thread_start() -> Option<Self::ThreadStartInfo> {
        None
    }

    /// Called before processing each message.
    ///
    /// Use this hook to enable per-item tracing or perform setup.
    #[allow(unused_variables)]
    fn pool_item_pre_process(pool_item_id: u64, thread_start_info: &mut Self::ThreadStartInfo) {
        // do nothing by default
    }

    /// Called after processing each message.
    ///
    /// Use this hook to disable per-item tracing or perform cleanup.
    #[allow(unused_variables)]
    fn pool_item_post_process(pool_item_id: u64, thread_start_info: &mut Self::ThreadStartInfo) {
        // do nothing by default
    }

    /// Determines which thread handles a given pool item ID.
    ///
    /// The default implementation is `id % thread_count`, which distributes IDs
    /// evenly across threads assuming sequential ID assignment.
    ///
    /// Override this for custom routing strategies (e.g., hash-based distribution
    /// for non-sequential IDs).
    fn id_thread_router(id: u64, thread_count: usize) -> u64 {
        id % (thread_count as u64)
    }
}
