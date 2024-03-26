use std::sync::Arc;

use tracing::{event, Level, Subscriber};

use super::{randoms_api::RandomsApi, Randoms};
use crate::{
    samples::{MeanResponse, RandomsAddRequest, SumResponse},
    *,
};

/// The implementation of this trait allows the Randoms struct to be used in the thread pool infrastructure
impl PoolItem for Randoms {
    type Init = RandomsAddRequest;
    type Api = RandomsApi;

    fn name() -> &'static str {
        "Randoms"
    }

    fn process_message(&mut self, request: Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsApi::Mean(request) => {
                event!(
                    Level::INFO,
                    "processing mean request for id {:?}",
                    id_being_processed()
                );
                MeanResponse {
                    id: request.id(),
                    mean: self.mean(),
                }
            }
            .into(),
            RandomsApi::Sum(request) => {
                event!(
                    Level::INFO,
                    "processing sum request for id {:?}",
                    id_being_processed()
                );
                SumResponse {
                    id: request.id(),
                    sum: self.sum(),
                }
            }
            .into(),
            RandomsApi::Panic(_request) => panic!("request to panic received"),
        }
    }

    /// This function (optionally) returns a tracing subscriber that should be used solely for tracing a
    /// given pool item
    /// If None then the thread subscriber will be used
    fn pool_item_subscriber(&self) -> Option<Arc<dyn Subscriber + Send + Sync>> {
        Self::randoms_tracing(self.id())
    }

    // fn add_pool_item_tracing(&self) -> Option<Vec<Box<dyn GuardDrop>>> {
    //     Self::randoms_tracing(self.id())
    // }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(Randoms::new(request.0))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
