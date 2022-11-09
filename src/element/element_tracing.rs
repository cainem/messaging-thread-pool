use tracing::subscriber::DefaultGuard;
use tracing_appender::non_blocking::WorkerGuard;

/// This trait needs to be implemented by Elements.\
/// It provides a hook to optionally provide tracing based on id.
/// The default implementation adds no tracing.
pub trait ElementTracing {
    /// This method is called to optionally add tracing before each message is processed.
    /// The tracing is removed once the message is processed.
    /// If the tracing is being written to a file it is important that the file is not truncated
    #[allow(unused_variables)]
    fn add_element_request_tracing(id: u64) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }

    /// This method provides any required tracing in the elements thread pool threads
    /// This tracing is added when the thread is spawned and remains in place until the thread dies
    #[allow(unused_variables)]
    fn add_pool_thread_tracing(id: u64) -> Option<(DefaultGuard, Vec<WorkerGuard>)> {
        None
    }
}
