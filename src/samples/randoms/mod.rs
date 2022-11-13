pub mod message_processor;
pub mod pool_item;
pub mod randoms_api;
pub mod randoms_request;
pub mod randoms_response;

use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

use crate::{
    element::{element_factory::ElementFactory, element_tracing::ElementTracing, Element},
    id_targeted::IdTargeted,
    thread_response::ThreadShutdownResponse,
};

use {
    randoms_request::RandomsRequest,
    randoms_response::{randoms_init_response::RandomsInitResponse, RandomsResponse},
};

/// This represents a simple collection of random numbers which is hosted inside the thread pool
///
/// It is tied to a particular thread by the modulus of its id.
///
/// The interface that it supports is governed by its implementation of the ElementProcess trait.
/// This in turn needs to be supported by the use of two enums of supported requests and responses
///
/// It supports the following operations
/// Init       creates a new Random with an stack based store of random numbers
/// Mean    calculates the mean of the contained numbers
/// Sum     calculates the sum of the contained numbers
#[derive(Debug, PartialEq, Eq)]
pub struct Randoms {
    pub id: u64,
    pub numbers: Vec<u64>,
}

impl Randoms {
    pub fn new(id: u64) -> Self {
        let mut rng = Xoshiro256Plus::seed_from_u64(id);
        let numbers = (0..10000).map(|_| rng.next_u64()).collect();
        Self { id, numbers }
    }

    pub fn mean(&self) -> u128 {
        self.numbers.iter().map(|n| *n as u128).sum::<u128>() / self.numbers.len() as u128
    }

    #[no_mangle]
    pub fn sum(&self) -> u128 {
        // do this very slowly with unnecessary loops
        for i in 0..=50 {
            let r = self.numbers.iter().map(|n| *n as u128).sum::<u128>();
            if i == 50 {
                return r;
            }
        }
        0
    }
}

impl ElementFactory<RandomsRequest, RandomsResponse> for Randoms {
    #[inline(always)]
    fn new_element(request: &RandomsRequest) -> (Option<Self>, RandomsResponse) {
        match request {
            RandomsRequest::Init(init) => (
                Some(Randoms::new(init.id)),
                RandomsResponse::Init(RandomsInitResponse { id: init.id }),
            ),
            _ => panic!("expected init only"),
        }
    }
}

impl Element for Randoms {
    type Request = RandomsRequest;
    type Response = RandomsResponse;

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // to mock shutdown (for test purposes) return id
        // as Randoms contains no child threads it really should not be
        // overridden (or it should return an empty vec)
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

impl IdTargeted for Randoms {
    fn id(&self) -> u64 {
        self.id
    }
}

impl ElementTracing for Randoms {}
