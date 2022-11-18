use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem,
    request_response::request_response_message::RequestResponseMessage,
    thread_request_response::ThreadRequestResponse,
};

use super::thread_pool_batcher_concrete::ThreadPoolBatcherConcrete;

impl<P> ThreadPoolBatcherConcrete<P>
where
    P: PoolItem,
{
    /// This function adds a single request to a building batch of request that will be sent at
    /// some future time when send_batch is called
    pub fn batch_for_send<const N: usize, U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequestResponse<P>> + IdTargeted + RequestResponseMessage<N, true>,
    {
        todo!();
        // self.to_send().borrow_mut().push(request.into());

        // self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{samples::*, thread_pool_batcher::ThreadPoolBatcherConcrete, ThreadPool};

    #[test]
    fn todo() {
        todo!();
    }

    // #[test]
    // fn batch_for_send_adds_request_to_internal_vec() {
    //     let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));
    //     let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

    //     let request = mean_request::MeanRequest { id: 1 };
    //     target.batch_for_send(request.clone());

    //     assert_eq!(1, target.to_send().borrow().len());
    //     assert_eq!(
    //         ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Mean(request)),
    //         target.to_send().borrow()[0]
    //     );
    // }
}
