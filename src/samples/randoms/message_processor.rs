use crate::{element::message_processor::MessageProcessor, id_targeted::IdTargeted};

use super::{
    randoms_request::RandomsRequest,
    randoms_response::{mean_response::MeanResponse, sum_response::SumResponse, RandomsResponse},
    Randoms,
};

/// The implementation of this trait defines the supported interface i.e. the operations that can be routed
/// to the underlying element, in this case a Randoms struct
///
/// Each request is expected to match a branch of processing within the process message
/// It is expected to return a response for every request.
/// The only exception is the request to create a new Random. This message is processed elsewhere
/// and should never reach the process_message function
impl MessageProcessor<RandomsRequest, RandomsResponse> for Randoms {
    fn process_message(&mut self, request: &RandomsRequest) -> RandomsResponse {
        match request {
            RandomsRequest::Mean(get_mean_request) => MeanResponse {
                id: get_mean_request.id,
                mean: self.mean(),
            }
            .into(),
            RandomsRequest::Sum(_get_state) => SumResponse {
                id: self.id,
                sum: self.sum(),
            }
            .into(),
            // process_message is called when a message arrives processing an existing element
            // The init message is for creating new elements and therefore should never turn up here
            RandomsRequest::Init(_) => panic!(
                "trying to create a key that already exists {}",
                request.get_id()
            ),
        }
    }
}
