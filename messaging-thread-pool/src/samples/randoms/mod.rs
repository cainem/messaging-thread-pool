use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use tracing::{Level, event};

use crate::IdTargeted;
use crate::ThreadShutdownResponse;
use crate::pool_item;

/// A collection of random numbers managed by the thread pool.
///
/// This sample demonstrates:
/// - CPU-intensive operations (seeded random number generation)
/// - Custom shutdown handling via `Shutdown = "shutdown_pool_impl"`
/// - Tracing integration with `tracing::event!`
/// - Benchmarking patterns (used in `benches/`)
///
/// # Generated Types
///
/// - `RandomsInit(u64)` / `RandomsAddRequest` - Create new Randoms with given ID
/// - `MeanRequest(u64)` / `MeanResponse` - Calculate mean of contained numbers
/// - `SumRequest(u64)` / `SumResponse` - Calculate sum of contained numbers
/// - `PanicRequest(u64)` / `PanicResponse` - Intentionally panic (for testing)
///
/// # Example
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::*};
///
/// let pool = ThreadPool::<Randoms>::new(4);
///
/// // Create item with ID 1
/// pool.send_and_receive_once(RandomsAddRequest(1)).unwrap();
///
/// // Calculate mean
/// let mean = pool.send_and_receive_once(MeanRequest(1)).unwrap();
/// println!("Mean: {}", mean.mean());
///
/// // Calculate sum
/// let sum = pool.send_and_receive_once(SumRequest(1)).unwrap();
/// println!("Sum: {}", sum.sum());
/// ```
///
/// # Shutdown Handling
///
/// This sample uses the `Shutdown` parameter to define a custom shutdown handler:
///
/// ```rust,ignore
/// #[pool_item(Shutdown = "shutdown_pool_impl")]
/// impl Randoms {
///     pub fn shutdown_pool_impl(&self) -> Vec<ThreadShutdownResponse> {
///         vec![ThreadShutdownResponse::new(self.id, vec![])]
///     }
/// }
/// ```
///
/// # Note
///
/// This is primarily used for benchmarking. For a simpler example, see [`ChatRoom`](super::ChatRoom).
/// For advanced patterns, see [`RandomsBatch`](super::RandomsBatch).
#[derive(Debug, PartialEq, Eq)]
pub struct Randoms {
    pub id: u64,
    pub numbers: Vec<u64>,
}

impl IdTargeted for Randoms {
    fn id(&self) -> u64 {
        self.id
    }
}

#[pool_item(Shutdown = "shutdown_pool_impl")]
impl Randoms {
    pub fn new(id: u64) -> Self {
        let mut rng = Xoshiro256Plus::seed_from_u64(id);
        let numbers = (0..10000).map(|_| rng.next_u64()).collect();
        Self { id, numbers }
    }

    pub fn shutdown_pool_impl(&self) -> Vec<ThreadShutdownResponse> {
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }

    #[messaging(MeanRequest, MeanResponse)]
    pub fn mean(&self) -> u128 {
        event!(Level::DEBUG, "evaluating mean");
        self.numbers.iter().map(|n| *n as u128).sum::<u128>() / self.numbers.len() as u128
    }

    #[unsafe(no_mangle)]
    #[messaging(SumRequest, SumResponse)]
    pub fn sum(&self) -> u128 {
        event!(Level::DEBUG, "evaluating sum");
        // do this very slowly with unnecessary loops
        for i in 0..=50 {
            let r = self.numbers.iter().map(|n| *n as u128).sum::<u128>();
            if i == 50 {
                return r;
            }
        }
        0
    }

    #[messaging(PanicRequest, PanicResponse)]
    pub fn panic_call(&self) {
        panic!("request to panic received")
    }
}

/// Alias for backwards compatibility
pub use RandomsInit as RandomsAddRequest;

impl MeanResponse {
    /// Get the calculated mean value
    pub fn mean(&self) -> u128 {
        self.result
    }
}

impl SumResponse {
    /// Get the calculated sum value
    pub fn sum(&self) -> u128 {
        self.result
    }
}
