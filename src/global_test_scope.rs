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

pub fn test_scope(filter: LevelFilter, enclosed_function: fn() -> ()) {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than LevelFilter
        // will be written to stdout.
        .with_max_level(filter)
        //.without_time()
        // completes the builder.
        .finish();

    let dispatcher = Dispatch::new(subscriber);

    tracing::dispatcher::with_default(&dispatcher, || {
        // Any trace events generated in this closure or by functions it calls
        // will be collected by `my_subscriber`.
        enclosed_function()
    });
}
