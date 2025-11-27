# messaging_thread_pool


[![Crates.io](https://img.shields.io/crates/v/once_cell.svg)](https://crates.io/crates/messaging_thread_pool)
[![API reference](https://docs.rs/messaging_thread_pool/badge.svg)](https://docs.rs/messaging_thread_pool/)

# Overview

`messaging_thread_pool` provides a set traits and structs that allows the construction of a simple typed thread pool.

It is useful when the type that needs to be distributed has complex state that is not send/sync.\
If the state is send and sync then it is probably better to use a more conventional thread pool such as rayon.\
Instances of the type are distributed across the threads of the thread pool and are tied to their allocated thread for their entire lifetime.\
Hence instances <b>do not need to be send nor sync</b> (although the messages used to communicate with them do).

## Key Advantages

1.  **Ownership of Non-Send/Sync Data**:
    Unlike traditional thread pools (e.g., `rayon`) where closures and data often need to be `Send` and `Sync` to move between threads, `messaging_thread_pool` guarantees that a `PoolItem` stays on the thread where it was created. This allows it to own:
    *   `Rc<T>` and `RefCell<T>` types.
    *   Raw pointers or FFI resources that are thread-bound.
    *   Large stack-allocated data structures that you don't want to move.

2.  **Stateful Long-Lived Objects (Actors)**:
    This library implements an Actor-like model. Items have an identity (`id`) and persistent state. You can send multiple messages to the same item over time, and it will maintain its state between requests. This is distinct from data-parallelism libraries which typically focus on stateless or shared-state parallel processing.

3.  **Sequential Consistency**:
    Messages sent to a specific `PoolItem` are processed sequentially in the order they are received. This eliminates race conditions within the item's state and simplifies reasoning about state transitions (e.g., ensuring "Initialize" happens before "Update").

4.  **Zero Contention & Lock-Free State**:
    Since only one thread ever accesses a specific `PoolItem`, there is no need for internal locking (Mutex/RwLock). You avoid the performance penalty of lock contention, even under heavy load.

5.  **Data Locality**:
    By pinning an item to a specific thread, its data remains in the CPU cache associated with that thread's core. This "warm cache" effect can significantly improve performance for state-heavy objects compared to work-stealing pools where tasks (and data) migrate between cores.

6.  **Message-Passing Architecture**:
    Communication happens via typed Request/Response messages. This decouples the caller from the execution details and fits naturally with the actor model.

7.  **Fine-Grained Concurrency**:
    You can target specific items by their ID. The pool handles the routing, ensuring that messages for the same ID are processed by the correct thread.

The library infrastructure then allows the routing of messages to specific instances based on a key.\
Any work required to respond to a message is executed on that instances assigned thread pool thread.\
Response messages are then routed back to the caller via the infrastructure.

It provides simple call schematics, easy to reason about lifetimes and predictable pool behaviour. 


The type needs to define an enum of message types and provide implementations of a few simple traits to enable it to be
hosted within the thread pool.

The `#[pool_item]` macro simplifies this process significantly.

### Example: Shared State without Locks

This example demonstrates a key advantage of this library: using `Rc` and `RefCell` to share state between a parent object and a helper struct. In a traditional thread pool, this would require `Arc<Mutex<...>>`.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use messaging_thread_pool_macros::pool_item;
use messaging_thread_pool::{IdTargeted, ThreadPool};

// A helper struct that needs access to the session's data.
// In a standard thread pool, this would likely need Arc<Mutex<Vec<String>>>.
// Here, we can use Rc<RefCell<...>> because UserSession never leaves its thread.
#[derive(Debug, Clone)]
struct HistoryTracker {
    // Shared access to the history log
    log: Rc<RefCell<Vec<String>>>,
}

impl HistoryTracker {
    fn add_entry(&self, entry: String) {
        // No locks! Just borrow_mut().
        self.log.borrow_mut().push(entry);
    }
}

// The main PoolItem
#[derive(Debug)]
pub struct UserSession {
    id: u64,
    // We hold the data
    log: Rc<RefCell<Vec<String>>>,
    // Our helper also holds a reference to the SAME data
    tracker: HistoryTracker,
}

impl IdTargeted for UserSession {
    fn id(&self) -> u64 {
        self.id
    }
}

#[pool_item]
impl UserSession {
    pub fn new(id: u64) -> Self {
        let log = Rc::new(RefCell::new(Vec::new()));
        let tracker = HistoryTracker { log: log.clone() };
        
        Self {
            id,
            log,
            tracker,
        }
    }

    #[messaging(LogActionRequest, LogActionResponse)]
    pub fn log_action(&self, action: String) -> usize {
        // We use the helper to modify the state
        self.tracker.add_entry(format!("Action: {}", action));
        
        // We can read the state directly
        self.log.borrow().len()
    }

    #[messaging(GetLogRequest, GetLogResponse)]
    pub fn get_log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }
}
```

With this infrastructure in place, a pool item can then use the library provided structs 
to host instances of the pool items in a fixed sized thread pool. 

```rust
// Create a thread pool with 2 threads
let thread_pool = ThreadPool::<UserSession>::new(2);

// Create a session with ID 1
thread_pool
    .send_and_receive(vec![UserSessionInit(1)].into_iter())
    .expect("session creation")
    .for_each(|_| {});

// Send some actions
// Note: These are processed sequentially by the thread owning Session 1
let counts: Vec<usize> = thread_pool
    .send_and_receive(vec![
        LogActionRequest(1, "Login".to_string()),
        LogActionRequest(1, "ViewProfile".to_string()),
        LogActionRequest(1, "Logout".to_string()),
    ].into_iter())
    .expect("actions")
    .map(|resp| resp.result)
    .collect();

assert_eq!(counts, vec![1, 2, 3]);

// Verify the log
let log = thread_pool
    .send_and_receive(vec![GetLogRequest(1)].into_iter())
    .expect("get log")
    .next()
    .unwrap()
    .result;

assert_eq!(log[0], "Action: Login");
assert_eq!(log[1], "Action: ViewProfile");
assert_eq!(log[2], "Action: Logout");
```

The original motivation for the library was to cope with hierarchies of long-lived dependent objects, each of which were required to have their own thread pools to avoid any complex threading dependencies.
All of the operations were CPU bound.

It is important to note that unless the operations being performed are quite long running (>50ms) then the costs of messaging infrastructure starts to become significant and will start to eat into the benefits of having multiple threads


