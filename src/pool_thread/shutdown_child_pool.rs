use crate::{
    pool_item::PoolItem, thread_request_response::thread_shutdown_response::ThreadShutdownResponse,
};

use super::PoolThread;

impl<E> PoolThread<E>
where
    E: PoolItem,
{
    /// This function attempts to close down any (child) thread pool that is associated with the elements
    /// in this thread pool
    pub fn shutdown_child_pool(&mut self) -> Vec<ThreadShutdownResponse> {
        // all elements should, if they contain a reference to a thread pool, have the ability to shut
        // it down, so just take the last one (so as to drop all of the contained elements) and call shutdown_child_threads
        if let Some((_, pool_item)) = self.element_hash_map.drain().last() {
            pool_item.shutdown_pool()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::{bounded, unbounded};

    use crate::samples::*;

    #[test]
    fn todo() {
        todo!();
    }

    // #[test]
    // fn thread_pool_contains_single_element_2() {
    //     let (_send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

    //     // request/response channel
    //     let (_send_back, _receive_back_from) = bounded::<ThreadResponse<RandomsResponse>>(0);

    //     let mut target = PoolThread::<Randoms>::new(1, receive_from_caller);

    //     let sample_element = Randoms {
    //         id: 2,
    //         numbers: vec![1, 2],
    //     };

    //     target.element_hash_map.insert(2, sample_element);

    //     let result = target.shutdown_child_pool();

    //     assert!(target.element_hash_map.is_empty());
    //     assert_eq!(1, result.len());
    //     assert_eq!(ThreadShutdownResponse::new(2, vec![]), result[0]);
    // }

    // #[test]
    // fn thread_pool_contains_single_element_1() {
    //     let (_send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

    //     // request/response channel
    //     let (_send_back, _receive_back_from) = bounded::<ThreadResponse<RandomsResponse>>(0);

    //     let mut target = PoolThread::<Randoms>::new(1, receive_from_caller);

    //     let sample_element = Randoms {
    //         id: 1,
    //         numbers: vec![100, 200],
    //     };

    //     target.element_hash_map.insert(1, sample_element);

    //     let result = target.shutdown_child_pool();

    //     assert!(target.element_hash_map.is_empty());
    //     assert_eq!(1, result.len());
    //     assert_eq!(ThreadShutdownResponse::new(1, vec![]), result[0]);
    // }

    // #[test]
    // fn thread_pool_contains_no_elements_shutdown_returns_empty_vec() {
    //     let (_send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

    //     // request/response channel
    //     let (_send_back, _receive_back_from) = bounded::<ThreadResponse<RandomsResponse>>(0);

    //     let mut target = PoolThread::<Randoms>::new(1, receive_from_caller);

    //     let result = target.shutdown_child_pool();

    //     assert!(target.element_hash_map.is_empty());
    //     assert!(result.is_empty());
    // }
}
