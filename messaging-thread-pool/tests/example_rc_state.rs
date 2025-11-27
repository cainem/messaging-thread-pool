use messaging_thread_pool::*;
use messaging_thread_pool_macros::pool_item;
use std::cell::RefCell;
use std::rc::Rc;

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

// This impl is required by the macro but the logic is handled by the macro
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

        Self { id, log, tracker }
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

#[test]
pub fn example_rc_state_management() {
    // We create a pool for our UserSessions
    let thread_pool = ThreadPool::<UserSession>::new(2);

    // Create a session with ID 1
    thread_pool
        .send_and_receive(vec![UserSessionInit(1)].into_iter())
        .expect("session creation")
        .for_each(|_| {});

    // Send some actions
    // Note: These are processed sequentially by the thread owning Session 1
    let counts: Vec<usize> = thread_pool
        .send_and_receive(
            vec![
                LogActionRequest(1, "Login".to_string()),
                LogActionRequest(1, "ViewProfile".to_string()),
                LogActionRequest(1, "Logout".to_string()),
            ]
            .into_iter(),
        )
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
}
