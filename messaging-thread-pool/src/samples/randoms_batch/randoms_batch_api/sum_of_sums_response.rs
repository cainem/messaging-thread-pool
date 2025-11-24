use crate::{samples::RandomsBatch, *};
use std::fmt::Debug;

use super::{InnerThreadPool, RandomsBatchApi};

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

impl<P: InnerThreadPool> From<SumOfSumsResponse> for ThreadRequestResponse<RandomsBatch<P>> {
    fn from(response: SumOfSumsResponse) -> Self {
        ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Response(response),
        ))
    }
}

impl<P: InnerThreadPool> From<ThreadRequestResponse<RandomsBatch<P>>> for SumOfSumsResponse {
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
            RequestResponse::Response(response),
        )) = response
        else {
            panic!("unexpected")
        };
        response
    }
}
