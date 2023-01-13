mod pool_item;
pub mod randoms_batch_api;

use std::sync::Arc;

use super::{RandomsAddRequest, RandomsBatchAddRequest, SumRequest, SumResponse};
use crate::{id_provider::IdProvider, samples::Randoms, *};

/// An example of an element that contains a child thread pool
///
/// RandomsBatches and Randoms form a hierarchy.
/// A RandomsBatch contains many Randoms.
///
/// RandomsBatches are managed by a one thread pool and internally they have a collection of Randoms which
/// are managed in a separate "child" thread pool
/// In this example all of the Randoms share a single thread pool regardless of which RandomsBatch created them
///
/// For this reason the RandomsBatches need to share an id_provider which provides globally unique ids
/// (ids, must be unique across the thread pool for obvious reasons)
#[derive(Debug)]
pub struct RandomsBatch<P>
where
    P: SenderAndReceiver<Randoms>,
{
    pub id: usize,
    pub contained_random_ids: Vec<usize>,
    pub id_provider: Arc<dyn IdProvider>,
    pub randoms_thread_pool: Arc<P>,
}

impl<P> RandomsBatch<P>
where
    P: SenderAndReceiver<Randoms> + Send + Sync,
{
    pub fn new(add_request: RandomsBatchAddRequest<P>) -> Self {
        let mut new = Self {
            id: add_request.id,
            contained_random_ids: vec![],
            id_provider: Arc::clone(&add_request.id_provider),
            randoms_thread_pool: Arc::clone(&add_request.randoms_thread_pool),
        };

        let mut ids = Vec::<usize>::default();
        new.randoms_thread_pool()
            .send_and_receive((0..add_request.number_of_contained_randoms).map(RandomsAddRequest))
            .expect("randoms thread pool to be available")
            .for_each(|r: AddResponse| {
                assert!(r.success(), "Request to add Randoms failed");
                ids.push(r.id());
                //new.contained_random_ids_mut().push(r.id());
            });

        new.contained_random_ids_mut().append(&mut ids);
        new
    }

    pub fn randoms_thread_pool(&self) -> &P {
        self.randoms_thread_pool.as_ref()
    }

    pub fn sum_of_sums(&self) -> u128 {
        // to get the sum of sums need to message the controls Randoms to get their sums
        // and then add them all up
        self.randoms_thread_pool()
            .send_and_receive(self.contained_random_ids.iter().map(|id| SumRequest(*id)))
            .expect("randoms thread pool to be available")
            .map(|response: SumResponse| response.sum())
            .sum()
    }

    pub fn contained_random_ids_mut(&mut self) -> &mut Vec<usize> {
        &mut self.contained_random_ids
    }
}
