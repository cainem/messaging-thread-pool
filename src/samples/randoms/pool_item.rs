use self::id_based_blocking::IdBasedBlocking;

use super::{randoms_api::RandomsApi, Randoms};
use crate::{
    samples::{MeanResponse, RandomsAddRequest, SumResponse},
    *,
};
use tracing::event;
use tracing_core::{Level, LevelFilter};

/// The implementation of this trait allows the Randoms struct to be used in the thread pool infrastructure
impl PoolItem for Randoms {
    type Init = RandomsAddRequest;
    type Api = RandomsApi;
    type ThreadStartInfo = IdBasedBlocking;

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

    fn name() -> &'static str {
        "Randoms"
    }

    fn thread_start() -> Option<Self::ThreadStartInfo> {
        Some(IdBasedBlocking::new("file"))
    }

    fn loading_pool_item(
        &self,
        pool_item_id: usize,
        thread_start_info: &mut Self::ThreadStartInfo,
    ) {
        match pool_item_id % 2 {
            0 => {
                thread_start_info
                    .set_level_and_id(LevelFilter::DEBUG, pool_item_id)
                    .expect("set level to work");
            }
            1 => {
                thread_start_info
                    .set_level_and_id(LevelFilter::OFF, pool_item_id)
                    .expect("set level to work");
            }
            _ => unreachable!(),
        }
    }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(Randoms::new(request.0))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
