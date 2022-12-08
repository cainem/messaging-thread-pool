mod send;

use std::thread::JoinHandle;

use crossbeam_channel::Sender;

use crate::{pool_item::PoolItem, sender_couplet::SenderCouplet};

/// A thread endpoint represents a thread within a thread pool
///
/// It consists of a channel to make requests on and a join handle
#[derive(Debug)]
pub(crate) struct ThreadEndpoint<P>
where
    P: PoolItem,
{
    pub sender: Sender<SenderCouplet<P>>,
    pub join_handle: JoinHandle<usize>,
}
