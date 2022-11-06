use crate::{element::Element, id_targeted::IdTargeted, thread_request::ThreadRequest};

use super::thread_pool_batcher_concrete::ThreadPoolBatcherConcrete;

impl<E> ThreadPoolBatcherConcrete<E>
where
    E: Element,
{
    /// This function adds a single request to a building batch of request that will be sent at
    /// some future time when send_batch is called
    pub fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequest<E::Request>> + IdTargeted,
    {
        self.to_send().borrow_mut().push(request.into());

        self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        samples::randoms::{
            randoms_request::{mean_request::MeanRequest, RandomsRequest},
            Randoms,
        },
        thread_pool_batcher::ThreadPoolBatcherConcrete,
        thread_request::ThreadRequest,
        ThreadPool,
    };

    #[test]
    fn batch_for_send_adds_request_to_internal_vec() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));
        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let request = MeanRequest { id: 1 };
        target.batch_for_send(request.clone());

        assert_eq!(1, target.to_send().borrow().len());
        assert_eq!(
            ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Mean(request)),
            target.to_send().borrow()[0]
        );
    }
}
