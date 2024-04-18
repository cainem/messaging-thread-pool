use std::fs::OpenOptions;
use std::sync::Arc;

use tracing::level_filters::LevelFilter;
use tracing::{Dispatch, Level, Subscriber};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{FmtSubscriber, Layer};

use crate::reference_counted_layer::ReferenceCountedLayer;
use crate::subscriber_and_guard::SubscriberAndGuard;
use crate::{samples::RandomsBatch, SenderAndReceiver};

use super::Randoms;

impl<P> RandomsBatch<P>
where
    P: SenderAndReceiver<Randoms>,
{
    /// An example of how to add conditional logging for a specific pool item.
    ///
    /// In this case the pool item 950 will output tracing (at DEBUG and above) to the console
    pub(crate) fn randoms_batch_tracing<S>(id: usize) -> Option<ReferenceCountedLayer<S>>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        // this is an example of how the logging can be added for a specific pool item
        // (in this case the pool item with id 950)

        let (non_blocking_log, guard) = tracing_appender::non_blocking(
            OpenOptions::new()
                .create(true)
                .truncate(false)
                .append(true)
                .open(format!("d:/temp/logs/random_batch_{id}.txt"))
                .unwrap(),
        );

        let log_file_layer = tracing_subscriber::fmt::layer()
            //.with_span_events(FmtSpan::ENTER)
            //.fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
            .with_ansi(false)
            .with_writer(non_blocking_log)
            //.with_thread_ids(true)
            //.without_time()
            //.compact()
            //.with_filter(non_blocking_log);
            .with_filter(LevelFilter::DEBUG);

        Some(ReferenceCountedLayer {
            guard: Some(Box::new(guard)),
            layer: Arc::new(log_file_layer),
        })

        // if id == 1 {
        //     Some(Arc::new(
        //         FmtSubscriber::builder()
        //             // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        //             // will be written to stdout.
        //             .with_thread_ids(true)
        //             .with_max_level(Level::TRACE)
        //             // completes the builder.
        //             .finish(),
        //     ))
        // } else {
        //     // no logging added
        //     None
        // }
    }
}
