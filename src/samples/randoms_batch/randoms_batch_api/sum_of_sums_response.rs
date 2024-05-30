use crate::{
    samples::{Randoms, RandomsBatch},
    *,
};
use std::fmt::Debug;

use super::RandomsBatchApi;

/// This response is returned from a request to calculate the sum of sums of all contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsResponse {
    pub id: u64,
    pub sum_of_sums: u128,
}

impl SumOfSumsResponse {
    pub fn sum_of_sums(&self) -> u128 {
        self.sum_of_sums
    }
}

// tie the message to the pool item and api variant
bind_response_to_api!(
    SumOfSumsResponse,
    RandomsBatch<P>,
    RandomsBatchApi::SumOfSums,
    P: SenderAndReceiver<Randoms>);
