use super::{MeanResponse, RandomsApi};
use crate::{samples::Randoms, *};

/// This defines a request to calculate the mean of the contained randoms
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanRequest(pub u64);

/// all requests must be able to provide the id of the pool item that they are targeting
impl IdTargeted for MeanRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

bind_request_to_response!(MeanRequest, Randoms, RandomsApi::Mean, MeanResponse);
