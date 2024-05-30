use super::{RandomsApi, SumResponse};
use crate::{samples::Randoms, *};

#[derive(Debug, PartialEq, Eq)]
pub struct SumRequest(pub u64);

impl IdTargeted for SumRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

bind_request_to_response!(SumRequest, Randoms, RandomsApi::Sum, SumResponse);
