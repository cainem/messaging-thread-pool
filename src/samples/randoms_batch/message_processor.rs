use std::{thread, time::Duration};

use crate::{element::message_processor::MessageProcessor, id_targeted::IdTargeted};

use super::{
    randoms_batch_request::RandomsBatchRequest,
    randoms_batch_response::{sum_of_sums_response::SumOfSumsResponse, RandomsBatchResponse},
    RandomsBatch,
};

/// The implementation of this trait defines how RandomsBatch responds to the requests
/// it receives.
///
/// In this example RandomsBatch can only receive one type of message, which is a request
/// to calculate a sum of sums for all the the Randoms it contains
impl MessageProcessor<RandomsBatchRequest, RandomsBatchResponse> for RandomsBatch {
    fn process_message(&mut self, request: &RandomsBatchRequest) -> RandomsBatchResponse {
        match request {
            RandomsBatchRequest::SumOfSums(sum_of_sums_request) => {
                // request for a sum of sums
                // message all of the controlled children to get their sums
                let result = SumOfSumsResponse {
                    id: sum_of_sums_request.id,
                    sum: self.sum_of_sums(),
                }
                .into();
                // simulate a long delay here for testing purposes
                thread::sleep(Duration::from_millis(100));
                result
            }
            // process_message is called when a message arrives processing an existing element
            // The init message is for creating new elements and therefore should never turn up here
            RandomsBatchRequest::Init(_) => panic!(
                "trying to create a key that already exists {}",
                request.id()
            ),
        }
    }
}
