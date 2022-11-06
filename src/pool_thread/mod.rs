mod message_loop;
mod new;
mod shutdown_child_pool;

use std::collections::HashMap;

use crossbeam_channel::Receiver;

use crate::{element::Element, sender_couplet::SenderCouplet};

/// This structure corresponds to a single logical thread within the thread pool
///
/// It has an id, a channel to receive messages on and a hash map of keyed elements
/// that will get managed by a given instance of a logical thread
pub struct PoolThread<E>
where
    E: Element,
{
    id: u64, // this will correspond to the vec index in the containing ThreadPool
    pool_thread_receiver: Receiver<SenderCouplet<E>>, // the channel on which requests will be received
    element_hash_map: HashMap<u64, E>,
}
