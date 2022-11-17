use std::collections::HashMap;

use crossbeam_channel::Receiver;

use crate::{pool_item::PoolItem, sender_couplet::SenderCouplet};

use super::PoolThread;

impl<E> PoolThread<E>
where
    E: PoolItem,
{
    /// This function creates a new PoolThread
    /// This represents a single thread in the thread pool
    ///
    /// The element_hash_map is in essence all of the state of the thread.
    /// It contains an entry for each "element" that is being managed within the thread-pool
    /// The routing logic is such that the same element will always be handled by the same PoolThread.
    ///
    /// Messages are passed to the the PoolThread on the pool_thread_receiver channel.
    ///
    /// The PoolThread spins around its message_loop function processing messages until a request is
    /// received to shutdown.
    pub fn new(id: usize, pool_thread_receiver: Receiver<SenderCouplet<E>>) -> Self {
        Self {
            thread_id: id,
            pool_thread_receiver,
            element_hash_map: HashMap::default(),
        }
    }
}
