mod message_loop;
pub mod new;
pub mod shutdown_child_pool;

use std::collections::HashMap;

use crossbeam_channel::Receiver;

use crate::{pool_item::PoolItem, sender_couplet::SenderCouplet};

pub struct PoolThread2<E>
where
    E: PoolItem,
{
    id: u64, // this will correspond to the vec index in the containing ThreadPool
    pool_thread_receiver: Receiver<SenderCouplet<E>>, // the channel on which requests will be received
    element_hash_map: HashMap<u64, E>,
}
