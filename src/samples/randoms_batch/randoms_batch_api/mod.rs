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
/// api_specification!(pool_item: RandomsBatch<T>, api_name: RandomsBatchApi, add_request: RandomsBatchAddRequest<T>,
/// calls: [
///           { call_name: SumOfSums, request: SumOfSumsRequest, response: SumOfSumsResponse },
///], generics: T: InnerThreadPool);

/// Define a trait to reduce type complexity for inner thread pool
pub trait InnerThreadPool: Debug {
    // the "thread pool" is really anything that implements this trait
    type ThreadPool: SenderAndReceiver<Randoms> + Send + Sync + Debug;
}

/// define a struct to identify concrete type of thread pool
#[derive(Debug)]
pub struct RandomsThreadPool;
impl InnerThreadPool for RandomsThreadPool {
    type ThreadPool = ThreadPool<Randoms>;
}

/// implement InnerThreadPool trait for mock thread pool
impl<T: RequestWithResponse<Randoms> + Send + Sync> InnerThreadPool
    for SenderAndReceiverMock<Randoms, T>
where
    <T as request_with_response::RequestWithResponse<Randoms>>::Response: Send,
{
    type ThreadPool = SenderAndReceiverMock<Randoms, T>;
}

#[derive(Debug)]
pub enum RandomsBatchApi<P: InnerThreadPool> {
    SumOfSums(RequestResponse<RandomsBatch<P>, SumOfSumsRequest>),
}

impl<P: InnerThreadPool> IdTargeted for RandomsBatchApi<P> {
    fn id(&self) -> u64 {
        let RandomsBatchApi::SumOfSums(RequestResponse::Request(sum_of_sum_request)) = self else {
            panic!("id not required to be implemented for responses")
        };
        sum_of_sum_request.id()
    }
}

impl<P: InnerThreadPool> From<ThreadRequestResponse<RandomsBatch<P>>> for RandomsBatchApi<P> {
    fn from(response: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
        let ThreadRequestResponse::MessagePoolItem(result) = response else {
            panic!("must be a response to a call to the pool item")
        };
        result
    }
}

impl<P: InnerThreadPool> From<RandomsBatchApi<P>> for ThreadRequestResponse<RandomsBatch<P>> {
    fn from(request_response: RandomsBatchApi<P>) -> Self {
        ThreadRequestResponse::MessagePoolItem(request_response)
    }
}
