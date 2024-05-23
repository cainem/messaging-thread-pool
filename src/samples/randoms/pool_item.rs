use std::fs;

use self::id_based_blocking::IdBasedBlocking;
use super::{randoms_api::RandomsApi, Randoms};
use crate::{
    samples::{MeanResponse, RandomsAddRequest, SumResponse},
    *,
};
use tracing::event;
use tracing_core::Level;

/// The implementation of this trait allows the Randoms struct to be used in the thread pool infrastructure
impl PoolItem for Randoms {
    type Init = RandomsAddRequest;
    type Api = RandomsApi;
    type ThreadStartInfo = Option<IdBasedBlocking>;

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
        let _ = fs::create_dir_all("target\\tmp\\logs\\random");
        Some(None)
    }

    fn pool_item_pre_process(pool_item_id: usize, thread_start_info: &mut Self::ThreadStartInfo) {
        // only log debug messages for the random with id 950
        if pool_item_id == 950 {
            // add IdBasedBlocking tracer
            let mut tracer = IdBasedBlocking::new("target\\tmp\\logs\\random\\trace");
            tracer.set_id(pool_item_id);
            thread_start_info.replace(tracer);
        }
    }

    fn pool_item_post_process(_pool_item_id: usize, thread_start_info: &mut Self::ThreadStartInfo) {
        // drop any
        let _take_to_drop = thread_start_info.take();
    }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(Randoms::new(request.0))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
