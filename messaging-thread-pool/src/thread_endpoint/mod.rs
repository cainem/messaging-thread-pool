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
    sender: Sender<SenderCouplet<P>>,
    join_handle: JoinHandle<u64>,
}

impl<P> ThreadEndpoint<P>
where
    P: PoolItem,
{
    pub(crate) fn new(sender: Sender<SenderCouplet<P>>, join_handle: JoinHandle<u64>) -> Self {
        Self {
            sender,
            join_handle,
        }
    }

    pub(crate) fn join_handle(self) -> JoinHandle<u64> {
        self.join_handle
    }
}
