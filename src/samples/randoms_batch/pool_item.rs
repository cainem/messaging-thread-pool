use crate::{samples::Randoms, *};
use std::{fmt::Debug, fs};

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
        let _ = fs::create_dir_all("target\\tmp\\logs\\random_batch");

        Some(IdBasedBlocking::new(
            "target\\tmp\\logs\\random_batch\\trace",
        ))
    }

    fn pool_item_pre_process(pool_item_id: usize, thread_start_info: &mut Self::ThreadStartInfo) {
        // we are using an ID based blocking logger
        // this logs to a file with the id of the pool item
        // this logger is intended for single threaded environments
        // which is ok because in essence that's what we have here (within the thread pool)
        thread_start_info.set_id(pool_item_id)
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
