use tracing::{Dispatch, Level};
use tracing_subscriber::FmtSubscriber;

use crate::guard_drop::GuardDrop;

use super::Randoms;

impl Randoms {
    /// An example of how to add conditional logging for a specific pool element.
    ///
    /// In this case the element 950 will output tracing (at DEBUG and above) to the console
    pub(crate) fn randoms_tracing(id: usize) -> Option<Vec<Box<dyn GuardDrop>>> {
        if id == 950 {
            println!("adding logging for {id:?}");

            let mut guards: Vec<Box<dyn GuardDrop>> = vec![];
            let subscriber = FmtSubscriber::builder()
                // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
                // will be written to stdout.
                .with_thread_ids(true)
                .with_max_level(Level::DEBUG)
                // completes the builder.
                .finish();
            let dispatcher = Dispatch::new(subscriber);
            let default_guard = tracing::dispatcher::set_default(&dispatcher);

            guards.push(Box::new(default_guard));
            Some(guards)
        } else {
            // no logging added
            None
        }
    }
}
