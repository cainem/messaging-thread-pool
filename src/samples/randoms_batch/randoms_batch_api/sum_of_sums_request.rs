use crate::{
    samples::{Randoms, RandomsBatch},
    *,
};
use std::fmt::Debug;

use super::{RandomsBatchApi, SumOfSumsResponse};

/// This is the message that is sent to request that a given RandomsBatch calculates the sum of all of the
/// sums of its contained Randoms
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumOfSumsRequest(pub u64);

impl IdTargeted for SumOfSumsRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

bind_request_to_response!(
    SumOfSumsRequest,
    RandomsBatch<P>,
    RandomsBatchApi::SumOfSums,
    SumOfSumsResponse,
    P: SenderAndReceiver<Randoms>
);
