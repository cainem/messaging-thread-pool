mod randoms_batch_add_request;
mod sum_of_sums_request;
mod sum_of_sums_response;

pub use self::{
    randoms_batch_add_request::RandomsBatchAddRequest, sum_of_sums_request::SumOfSumsRequest,
    sum_of_sums_response::SumOfSumsResponse,
};
use crate::{samples::Randoms, *};
use std::fmt::Debug;

use super::RandomsBatch;

/// Define the RandomsBatchApi which defines the a request/response
/// pair that can be used to communicate with the RandomsBatch pool item
///
/// This sample has been implemented without the use of the api_specification
/// macro to demonstrate what code needs to be written without it.
/// A lot of this repetitive boiler plate code can be omitted if the macro is used.
///
/// This api could have been defined with this macro statement
///
/// use messaging_thread_pool::api_specification;
/// api_specification!(pool_item: Randoms, api_name: RandomsApi, add_request: RandomsAddRequest,
///     calls: [
///               { call_name: Mean, request: MeanRequest, response: MeanResponse },
///    ], trait_name);
///

#[derive(Debug)]
pub enum RandomsBatchApi<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    SumOfSums(RequestResponse<RandomsBatch<P>, SumOfSumsRequest>),
}

impl<P> IdTargeted for RandomsBatchApi<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    fn id(&self) -> u64 {
        let RandomsBatchApi::SumOfSums(RequestResponse::Request(sum_of_sum_request)) = self else {
            panic!("id not required to be implemented for responses")
        };
        sum_of_sum_request.id()
    }
}

impl<P> From<ThreadRequestResponse<RandomsBatch<P>>> for RandomsBatchApi<P>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(result) = response else {
            panic!("must be a response to a call to the pool item")
        };
        result
    }
}

impl<P> From<RandomsBatchApi<P>> for ThreadRequestResponse<RandomsBatch<P>>
where
    P: SenderAndReceiver<Randoms> + Send + Debug + Sync,
{
    fn from(request_response: RandomsBatchApi<P>) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
