use std::sync::Arc;

use tracing::{Level, Subscriber};
use tracing_subscriber::FmtSubscriber;

use super::Randoms;

impl Randoms {
    /// An example of how to add conditional logging for a specific pool item.
    ///
    /// In this case the pool item 950 will output tracing (at DEBUG and above) to the console
    pub(crate) fn randoms_tracing(id: usize) -> Option<Arc<dyn Subscriber + Send + Sync>> {
        // this is an example of how the logging can be added for a specific pool item
        // (in this case the pool item with id 950)
        if id == 950 {
            Some(Arc::new(
                FmtSubscriber::builder()
                    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
                    // will be written to stdout.
                    .with_thread_ids(true)
                    .with_max_level(Level::DEBUG)
                    // completes the builder.
                    .finish(),
            ))
        } else {
            // no logging added
            None
        }
    }
}
