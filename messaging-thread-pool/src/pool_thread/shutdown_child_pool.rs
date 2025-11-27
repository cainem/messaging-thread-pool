use crate::{pool_item::PoolItem, thread_request_response::*};

use super::PoolThread;

impl<P> PoolThread<P>
where
    P: PoolItem,
{
    /// This function attempts to close down any (child) thread pool that is associated with the pool items
    /// in this thread pool
    pub fn shutdown_child_pool(&mut self) -> Vec<ThreadShutdownResponse> {
        // all pool items should, if they contain a reference to a thread pool, have the ability to shut
        // it down, so just take the first one, clear the map of all others and call shutdown_child_threads
        if let Some(key) = self.pool_item_map.keys().min().copied() {
            let pool_item = self.pool_item_map.remove(&key).unwrap();
            self.pool_item_map.clear();
            pool_item.shutdown_pool()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::{bounded, unbounded};

    use crate::{
        pool_thread::PoolThread, samples::*, sender_couplet::SenderCouplet,
        thread_request_response::*,
    };

    #[test]
    fn thread_pool_contains_single_element_2() {
        let (_send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

        // request/response channel
        let (_send_back, _receive_back_from) = bounded::<ThreadRequestResponse<Randoms>>(0);

        let mut target = PoolThread::<Randoms>::new(1, receive_from_caller);

        let sample_pool_item = Randoms {
            id: 2,
            numbers: vec![1, 2],
        };

        target.pool_item_map.insert(2, sample_pool_item);

        let result = target.shutdown_child_pool();

        assert!(target.pool_item_map.is_empty());
        assert_eq!(1, result.len());
        assert_eq!(ThreadShutdownResponse::new(2, vec![]), result[0]);
    }

    #[test]
    fn thread_pool_contains_single_element_1() {
        let (_send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

        // request/response channel
        let (_send_back, _receive_back_from) = bounded::<ThreadRequestResponse<Randoms>>(0);

        let mut target = PoolThread::<Randoms>::new(1, receive_from_caller);

        let sample_pool_item = Randoms {
            id: 1,
            numbers: vec![100, 200],
        };

        target.pool_item_map.insert(1, sample_pool_item);

        let result = target.shutdown_child_pool();

        assert!(target.pool_item_map.is_empty());
        assert_eq!(1, result.len());
        assert_eq!(ThreadShutdownResponse::new(1, vec![]), result[0]);
    }

    #[test]
    fn thread_pool_contains_no_elements_shutdown_returns_empty_vec() {
        let (_send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

        // request/response channel
        let (_send_back, _receive_back_from) = bounded::<ThreadRequestResponse<Randoms>>(0);

        let mut target = PoolThread::<Randoms>::new(1, receive_from_caller);

        let result = target.shutdown_child_pool();

        assert!(target.pool_item_map.is_empty());
        assert!(result.is_empty());
    }
}
