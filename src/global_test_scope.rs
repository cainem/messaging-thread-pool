use tracing::{metadata::LevelFilter, Dispatch};
use tracing_subscriber::FmtSubscriber;

/// A trace helper used in the examples
pub fn global_test_scope(filter: LevelFilter) {
    let subscriber = FmtSubscriber::builder()
        .with_thread_ids(true)
        .with_max_level(filter)
        // completes the builder.
        .finish();
    let dispatcher = Dispatch::new(subscriber);
    tracing::dispatcher::set_global_default(dispatcher).unwrap();
}
