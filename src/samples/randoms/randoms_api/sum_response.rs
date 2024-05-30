use crate::{samples::Randoms, *};

use super::RandomsApi;

/// This is the response from a request to calculate the sum of the contained random numbers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumResponse {
    pub id: u64,
    pub sum: u128,
}

impl SumResponse {
    pub fn sum(&self) -> u128 {
        self.sum
    }
}

// tie the response to an api call
bind_response_to_api!(SumResponse, Randoms, RandomsApi::Sum);
