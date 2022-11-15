mod send;

use std::thread::JoinHandle;

use crossbeam_channel::Sender;

use crate::{pool_item::PoolItem, sender_couplet_2::SenderCouplet2};

/// A thread endpoint represents a thread within a thread pool
///
/// It consists of a channel to make requests on and a join handle
#[derive(Debug)]
pub struct ThreadEndpoint<E>
where
    E: PoolItem,
{
    pub sender: Sender<SenderCouplet2<E>>,
    pub join_handle: JoinHandle<u64>,
}
