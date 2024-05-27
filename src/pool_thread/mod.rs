mod message_loop;
pub mod new;
pub mod shutdown_child_pool;

use std::collections::BTreeMap;

use crossbeam_channel::Receiver;

use crate::{pool_item::PoolItem, sender_couplet::SenderCouplet};

/// This structure represents a thread within the thread pool
pub struct PoolThread<P>
where
    P: PoolItem,
{
    /// A unique id assigned to the pool thread
    thread_id: u64,
    /// Stores the channel on which requests will be received
    pool_thread_receiver: Receiver<SenderCouplet<P>>,
    /// This is a hash map that will hold the ownership of all pool items created in this
    /// pool thread keyed by their ids
    pool_item_map: BTreeMap<u64, P>,
}
