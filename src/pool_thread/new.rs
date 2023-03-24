use std::collections::BTreeMap;

use crossbeam_channel::Receiver;

use crate::{pool_item::PoolItem, sender_couplet::SenderCouplet};

use super::PoolThread;

impl<P> PoolThread<P>
where
    P: PoolItem,
{
    /// This function creates a new PoolThread
    /// This represents a single thread in the thread pool
    ///
    /// The pool_item_hash_map is in essence all of the state of the thread.
    /// It contains an entry for each pool item that is being managed within the thread-pool
    /// The routing logic is such that the same pool item will always be handled by the same PoolThread.
    ///
    /// Messages are passed to the the PoolThread on the pool_thread_receiver channel.
    ///
    /// The PoolThread spins around its message_loop function processing messages until a request is
    /// received to shutdown.
    pub(crate) fn new(id: usize, pool_thread_receiver: Receiver<SenderCouplet<P>>) -> Self {
        Self {
            thread_id: id,
            pool_thread_receiver,
            pool_item_map: BTreeMap::default(),
        }
    }
}
