use tracing_core::LevelFilter;

use crate::{samples::Randoms, *};
use std::fmt::Debug;

use self::id_based_blocking::IdBasedBlocking;

use super::{randoms_batch_api::*, RandomsBatch};

impl<P> PoolItem for RandomsBatch<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync + Debug,
{
    type Init = RandomsBatchAddRequest<P>;
    type Api = RandomsBatchApi<P>;
    type ThreadStartInfo = IdBasedBlocking;

    fn name() -> &'static str {
        "RandomsBatch"
    }

    fn thread_start() -> Option<Self::ThreadStartInfo> {
        Some(IdBasedBlocking::new(
            "d:\\temp\\logs\\random_batch\\random_batch",
        ))
    }

    fn loading_pool_item(
        &self,
        pool_item_id: usize,
        thread_start_info: &mut Self::ThreadStartInfo,
    ) {
        thread_start_info
            .set_level_and_id(LevelFilter::DEBUG, pool_item_id)
            .expect("set level to work");
    }

    fn process_message(&mut self, request: Self::Api) -> ThreadRequestResponse<Self> {
        match request {
            RandomsBatchApi::SumOfSums(RequestResponse::Request(request)) => {
                let id = request.id();
                let sum_of_sums = self.sum_of_sums();
                SumOfSumsResponse { id, sum_of_sums }.into()
            }
            _ => panic!("unexpected"),
        }
    }

    fn new_pool_item(request: Self::Init) -> Result<Self, NewPoolItemError> {
        Ok(RandomsBatch::new(request))
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        // for test purposes simulate shutting down a thread with the id of the randoms
        vec![ThreadShutdownResponse::new(self.id, vec![])]
    }
}
