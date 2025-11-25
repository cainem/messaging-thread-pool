use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use tracing::{Level, event};

use crate::pool_item;
use crate::IdTargeted;
use crate::ThreadShutdownResponse;

/// This represents a simple collection of random numbers which is hosted inside the thread pool
///
/// It is tied to a particular thread by the modulus of its id.
///
/// The interface that it supports is governed by its implementation of the PoolItem trait.
/// This in turn needs to be supported by the use of two enums of supported requests and responses
///
/// It supports the following operations
/// Init    creates a new Random with an stack based store of random numbers
/// Mean    calculates the mean of the contained numbers
/// Sum     calculates the sum of the contained numbers
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

pub use RandomsInit as RandomsAddRequest;

impl MeanResponse {
    pub fn mean(&self) -> u128 {
        self.result
    }
}

impl SumResponse {
    pub fn sum(&self) -> u128 {
        self.result
    }
}
