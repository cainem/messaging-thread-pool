use tracing::{metadata::LevelFilter, Dispatch};
use tracing_subscriber::FmtSubscriber;

pub fn global_test_scope(filter: LevelFilter) {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_thread_ids(true)
        .with_max_level(filter)
        // completes the builder.
        .finish();
    let dispatcher = Dispatch::new(subscriber);
    tracing::dispatcher::set_global_default(dispatcher).unwrap();
}
