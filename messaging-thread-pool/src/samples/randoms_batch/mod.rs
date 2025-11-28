//! # RandomsBatch - Advanced Nested Thread Pool Example
//!
//! This module demonstrates advanced patterns including:
//! - Generic pool items
//! - Custom initialization types
//! - Nested thread pools (a pool item that contains another pool)
//! - Mocking nested dependencies for testing

use std::fmt::Debug;
use std::sync::Arc;

use crate::pool_item;
use crate::samples::Randoms;
use crate::{id_provider::IdProvider, *};

use super::{RandomsAddRequest, SumRequest, SumResponse};

/// Trait for abstracting the inner thread pool type.
///
/// This enables:
/// - Using a real `ThreadPool<Randoms>` in production
/// - Using a `SenderAndReceiverMock` in tests
///
/// # Example
///
/// ```rust,ignore
/// // Production code uses RandomsThreadPool
/// let batch: RandomsBatch<RandomsThreadPool> = /* ... */;
///
/// // Test code uses a mock
/// let batch: RandomsBatch<SenderAndReceiverMock<_, _>> = /* ... */;
/// ```
pub trait InnerThreadPool: Debug + Send {
    /// The concrete thread pool type that implements `SenderAndReceiver<Randoms>`
    type ThreadPool: SenderAndReceiver<Randoms> + Send + Sync + Debug;
}

/// Marker type for using a real `ThreadPool<Randoms>` as the inner pool.
#[derive(Debug)]
pub struct RandomsThreadPool;
impl InnerThreadPool for RandomsThreadPool {
    type ThreadPool = ThreadPool<Randoms>;
}

/// Implement `InnerThreadPool` for mock types to enable testing.
impl<T: RequestWithResponse<Randoms> + Send + Sync> InnerThreadPool
    for SenderAndReceiverMock<Randoms, T>
where
    <T as request_with_response::RequestWithResponse<Randoms>>::Response: Send,
{
    type ThreadPool = SenderAndReceiverMock<Randoms, T>;
}

/// Custom initialization request for `RandomsBatch`.
///
/// This demonstrates using a custom `Init` type instead of the generated one.
/// Custom Init types are needed when:
/// - The constructor needs more than just an ID
/// - You need to pass complex configuration
/// - The pool item is generic and needs type information
///
/// # Fields
///
/// - `id` - Unique identifier for this batch
/// - `number_of_contained_randoms` - How many `Randoms` items to create
/// - `id_provider` - Shared ID generator (ensures unique IDs across batches)
/// - `randoms_thread_pool` - Shared inner thread pool for the `Randoms`
#[derive(Debug, Clone)]
pub struct RandomsBatchAddRequest<P: InnerThreadPool> {
    pub id: u64,
    pub number_of_contained_randoms: usize,
    pub id_provider: Arc<dyn IdProvider>,
    /// This thread pool is shared by all Randoms across all RandomsBatches
    pub randoms_thread_pool: Arc<P::ThreadPool>,
}

impl<P: InnerThreadPool> RandomsBatchAddRequest<P> {
    pub fn id_provider(&self) -> &dyn IdProvider {
        self.id_provider.as_ref()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl<P: InnerThreadPool> IdTargeted for RandomsBatchAddRequest<P> {
    fn id(&self) -> u64 {
        self.id
    }
}

impl<P: InnerThreadPool> RequestWithResponse<RandomsBatch<P>> for RandomsBatchAddRequest<P> {
    type Response = AddResponse;
}

impl<P: InnerThreadPool> From<RandomsBatchAddRequest<P>>
    for ThreadRequestResponse<RandomsBatch<P>>
{
    fn from(request: RandomsBatchAddRequest<P>) -> Self {
        ThreadRequestResponse::<RandomsBatch<P>>::AddPoolItem(RequestResponse::Request(request))
    }
}

/// A batch of `Randoms` items, demonstrating nested thread pools.
///
/// This is an advanced example showing:
/// - **Generic pool items**: `RandomsBatch<P>` is generic over the inner pool type
/// - **Custom Init type**: Uses `RandomsBatchAddRequest<P>` instead of generated init
/// - **Nested pools**: Each batch references an inner pool of `Randoms`
/// - **Shared resources**: Multiple batches share the same inner pool and ID provider
///
/// # Architecture
///
/// ```text
/// ┌─────────────────────────────────────────┐
/// │     ThreadPool<RandomsBatch<P>>         │ ← Outer pool (manages batches)
/// │  ┌─────────────┐  ┌─────────────┐       │
/// │  │ Batch (id=0)│  │ Batch (id=1)│  ...  │
/// │  │  refs→[1,2] │  │  refs→[3,4] │       │
/// │  └──────┬──────┘  └──────┬──────┘       │
/// └─────────┼────────────────┼──────────────┘
///           │                │
///           ▼                ▼
/// ┌─────────────────────────────────────────┐
/// │       Arc<ThreadPool<Randoms>>          │ ← Inner pool (shared)
/// │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐    │
/// │  │ R(1) │ │ R(2) │ │ R(3) │ │ R(4) │    │
/// │  └──────┘ └──────┘ └──────┘ └──────┘    │
/// └─────────────────────────────────────────┘
/// ```
///
/// # Example: Real Thread Pools
///
/// ```rust
/// use std::sync::Arc;
/// use messaging_thread_pool::{ThreadPool, id_provider::id_provider_mutex::IdProviderMutex, samples::*};
///
/// // Create the outer pool for batches
/// let batch_pool = ThreadPool::<RandomsBatch<RandomsThreadPool>>::new(2);
///
/// // Create the shared inner pool for Randoms
/// let randoms_pool = Arc::new(ThreadPool::<Randoms>::new(4));
///
/// // Create shared ID provider (ensures unique IDs across all batches)
/// let id_provider = Arc::new(IdProviderMutex::new(0));
///
/// // Create a batch - this will create 10 Randoms in the inner pool
/// batch_pool.send_and_receive_once(RandomsBatchAddRequest {
///     id: 0,
///     number_of_contained_randoms: 10,
///     id_provider: id_provider.clone(),
///     randoms_thread_pool: randoms_pool.clone(),
/// }).unwrap();
///
/// // Query the batch - it will in turn query its Randoms
/// let response = batch_pool.send_and_receive_once(
///     SumOfSumsRequest(0, std::marker::PhantomData)
/// ).unwrap();
/// ```
///
/// # Example: Mocking the Inner Pool
///
/// See `tests/example_two_level.rs` for a complete mocking example.
#[derive(Debug)]
pub struct RandomsBatch<P: InnerThreadPool> {
    pub id: u64,
    pub contained_random_ids: Vec<u64>,
    pub id_provider: Arc<dyn IdProvider>,
    pub randoms_thread_pool: Arc<P::ThreadPool>,
}

#[pool_item(Init = "RandomsBatchAddRequest<P>")]
impl<P: InnerThreadPool> RandomsBatch<P> {
    pub fn new(add_request: RandomsBatchAddRequest<P>) -> Self {
        let mut new = Self {
            id: add_request.id,
            contained_random_ids: vec![],
            id_provider: Arc::clone(&add_request.id_provider),
            randoms_thread_pool: Arc::clone(&add_request.randoms_thread_pool),
        };

        let mut ids = Vec::<u64>::default();
        new.randoms_thread_pool()
            .send_and_receive(
                (0..add_request.number_of_contained_randoms)
                    .map(|_| RandomsAddRequest(new.id_provider.next_id())),
            )
            .expect("randoms thread pool to be available")
            .for_each(|r: AddResponse| {
                assert!(r.result().is_ok(), "Request to add Randoms failed");
                ids.push(r.id());
            });

        new.contained_random_ids_mut().append(&mut ids);
        new
    }

    pub fn randoms_thread_pool(&self) -> &P::ThreadPool {
        self.randoms_thread_pool.as_ref()
    }

    #[messaging(SumOfSumsRequest, SumOfSumsResponse)]
    pub fn sum_of_sums(&self) -> u128 {
        // to get the sum of sums need to message the controls Randoms to get their sums
        // and then add them all up
        self.randoms_thread_pool()
            .send_and_receive(self.contained_random_ids.iter().map(|id| SumRequest(*id)))
            .expect("randoms thread pool to be available")
            .map(|response: SumResponse| response.sum())
            .sum()
    }

    pub fn contained_random_ids_mut(&mut self) -> &mut Vec<u64> {
        &mut self.contained_random_ids
    }
}
