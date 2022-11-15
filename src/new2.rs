// use std::{sync::RwLock, thread::spawn};

// use crossbeam_channel::unbounded;
// use tracing::{event, Level};

// use crate::{
//     pool_item::PoolItem, pool_thread_2::PoolThread2, sender_couplet_2::SenderCouplet2,
//     thread_endpoint_2::ThreadEndpoint2, thread_pool_2::ThreadPool2,
// };

// impl<E> ThreadPool2<E>
// where
//     // 'static - the Element cannot contain any references as it isn't guaranteed to live long enough
//     // due to it being passed to another thread
//     E: PoolItem + 'static,
// {
//     /// This function creates a new [`ThreadPool`]
//     ///
//     /// Internally it creates a collection of threads.
//     /// It has the ability to communicate with the threads via a vec of channels
//     /// (there is one channel for each spawned thread)
//     ///
//     /// The number of threads is determined by the passed in thread_pool_size
//     pub fn new2(thread_pool_size: usize) -> Self {
//         assert!(
//             thread_pool_size > 0,
//             "thread pool must have at least one thread"
//         );

//         let mut building = Vec::<ThreadEndpoint2<E>>::new();

//         for i in 0..thread_pool_size as u64 {
//             let (send_to_thread, receive_from_pool) = unbounded::<SenderCouplet2<E>>();

//             let join_handle = spawn(move || {
//                 // set default tracing subscribers for thread
//                 let tracing_guards = E::add_pool_thread_tracing(i);

//                 // start a new thread with id i
//                 let mut pool_thread = PoolThread2::<E>::new(i, receive_from_pool);

//                 event!(Level::INFO, "starting message loop");

//                 // enter the "infinite" message loop where messages will be received
//                 pool_thread.message_loop();

//                 // drop the threads tracing subscriber
//                 drop(tracing_guards);

//                 // return the pool thread id in the join handle
//                 i
//             });

//             building.push(ThreadEndpoint2 {
//                 sender: send_to_thread,
//                 join_handle,
//             });
//         }

//         ThreadPool2 {
//             thread_endpoints: RwLock::new(building),
//         }
//     }
// }
