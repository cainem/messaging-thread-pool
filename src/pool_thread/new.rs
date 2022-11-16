// use std::collections::HashMap;

// use crossbeam_channel::Receiver;

// use crate::{pool_item::PoolItem, sender_couplet::SenderCouplet};

// use super::PoolThread;

// impl<E> PoolThread<E>
// where
//     E: PoolItem,
// {
//     /// This function creates a new PoolThread
//     /// This represents a single thread in the thread pool
//     ///
//     /// The element_hash_map is in essence all of the state of the thread.
//     /// It contains an entry for each "element" that is being managed within the thread-pool
//     /// The routing logic is such that the same element will always be handled by the same PoolThread.
//     ///
//     /// Messages are passed to the the PoolThread on the pool_thread_receiver channel.
//     ///
//     /// The PoolThread spins around its message_loop function processing messages until a request is
//     /// received to shutdown.
//     pub fn new(id: u64, pool_thread_receiver: Receiver<SenderCouplet<E>>) -> Self {
//         Self {
//             id,
//             pool_thread_receiver,
//             element_hash_map: HashMap::default(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use crossbeam_channel::{bounded, unbounded};

//     use crate::{
//         pool_thread::PoolThread, samples::*, thread_request::ThreadRequest,
//         thread_response::ThreadResponse,
//     };

//     #[test]
//     fn todo() {
//         todo!();
//     }

//     // #[test]
//     // fn new_constructs_as_expected() {
//     //     // thread channel
//     //     let (send_to_thread, receive_from_caller) = unbounded::<SenderCouplet<Randoms>>();

//     //     // request/response channel
//     //     let (send_back, _receive_back_from) = bounded::<ThreadResponse<RandomsResponse>>(0);

//     //     let result = PoolThread::<Randoms>::new(1, receive_from_caller);

//     //     let request = ThreadRequest::ThreadEcho(1, "ping".to_string());
//     //     let message = SenderCouplet::new(send_back, request.clone());

//     //     send_to_thread.send(message).expect("send to work");

//     //     assert_eq!(1, result.id);
//     //     assert_eq!(HashMap::default(), result.element_hash_map);
//     //     assert_eq!(
//     //         &request,
//     //         result
//     //             .pool_thread_receiver
//     //             .recv()
//     //             .unwrap()
//     //             .get_thread_request()
//     //     );
//     // }
// }
