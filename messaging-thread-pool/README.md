# messaging_thread_pool


[![Crates.io](https://img.shields.io/crates/v/once_cell.svg)](https://crates.io/crates/messaging_thread_pool)
[![API reference](https://docs.rs/messaging_thread_pool/badge.svg)](https://docs.rs/messaging_thread_pool/)

# Overview

`messaging_thread_pool` provides a set traits and structs that allows the construction of a simple typed thread pool.

It is useful when the type that needs to be distributed has complex state that is not send/sync.\
If the state is send and sync then it is probably better to use a more conventional thread pool such as rayon.\
Instances of the type are distributed across the threads of the thread pool and are tied to their allocated thread for their entire lifetime.\
Hence instances <b>do not need to be send nor sync</b> (although the messages used to communicate with them do).

The library infrastructure then allows the routing of messages to specific instances based on a key.\
Any work required to respond to a message is executed on that instances assigned thread pool thread.\
Response messages are then routed back to the caller via the infrastructure.

It provides simple call schematics, easy to reason about lifetimes and predictable pool behaviour. 


The type needs to define an enum of message types and provide implementations of a few simple traits to enable it to be
hosted within the thread pool.

The `#[pool_item]` macro simplifies this process significantly.

So, for example, a simple type representing a Chat Room:

```rust
use messaging_thread_pool_macros::pool_item;

#[derive(Debug)]
pub struct ChatRoom {
    id: u64,
    pub history: Vec<String>,
}

impl ChatRoom {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            history: Vec::new(),
        }
    }
}

#[pool_item]
impl ChatRoom {
    #[messaging(PostRequest, PostResponse)]
    pub fn post(&mut self, user: String, text: String) -> usize {
        let entry = format!("{}: {}", user, text);
        self.history.push(entry);
        self.history.len() - 1
    }

    #[messaging(GetHistoryRequest, GetHistoryResponse)]
    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
}
```

With this infrastructure in place a pool item can then use the library provided structs 
to host instances of the pool items in a fixed sized thread pool. 

```rust
use messaging_thread_pool::ThreadPool;

// Create a thread pool with 4 threads
let thread_pool = ThreadPool::<ChatRoom>::new(4);

// Create two chat rooms (ID 1 and 2)
// The pool will route these to the appropriate threads based on ID
thread_pool
    .send_and_receive(vec![
        ChatRoomInit(1),
        ChatRoomInit(2),
    ].into_iter())
    .expect("creation requests")
    .for_each(|_| {});

// Post messages to Room 1
thread_pool
    .send_and_receive(vec![
        PostRequest(1, "Alice".to_string(), "Hello!".to_string()),
        PostRequest(1, "Bob".to_string(), "Hi Alice!".to_string()),
    ].into_iter())
    .expect("messages to send")
    .for_each(|response| {
        println!("Message index: {}", response.result);
    });

// Get history from Room 1
let history = thread_pool
    .send_and_receive(vec![GetHistoryRequest(1)].into_iter())
    .expect("request to send")
    .next()
    .expect("response")
    .result;

assert_eq!(history.len(), 2);
assert_eq!(history[0], "Alice: Hello!");
```

The original motivation for the library was to cope with hierarchies of long-lived dependent objects, each of which
were required to have their own thread pools to avoid any complex threading dependencies.
All of the operations were CPU bound.

It is important to note that unless the operations being performed are quite long running (>50ms) then the costs of
messaging infrastructure starts to become significant and will start to eat into the benefits of having multiple threads


