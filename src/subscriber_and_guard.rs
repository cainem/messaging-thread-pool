use tracing::Subscriber;

/// this struct wraps a subscriber and a guard together
/// this is necessary because quite often the creation of a subscriber will return a guard to an internal thread
/// so functions that do the "subscriber setup" will need to return both the subscriber and the guard
pub struct SubscriberAndGuard {
    subscriber: Box<dyn Subscriber + Send + Sync>,
    #[allow(dead_code)]
    guard: Box<dyn Send + Sync>,
}

impl SubscriberAndGuard {
    #[allow(dead_code)]
    pub fn new(subscriber: Box<dyn Subscriber + Send + Sync>, guard: Box<dyn Send + Sync>) -> Self {
        Self { subscriber, guard }
    }
}

/// this is a convenience implementation of the Subscriber trait for SubscriberAndGuard
impl Subscriber for SubscriberAndGuard {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        self.subscriber.enabled(metadata)
    }

    fn new_span(&self, span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        self.subscriber.new_span(span)
    }

    fn record(&self, span: &tracing::span::Id, values: &tracing::span::Record<'_>) {
        self.subscriber.record(span, values)
    }

    fn record_follows_from(&self, span: &tracing::span::Id, follows: &tracing::span::Id) {
        self.subscriber.record_follows_from(span, follows)
    }

    fn event(&self, event: &tracing::Event<'_>) {
        self.subscriber.event(event)
    }

    fn enter(&self, span: &tracing::span::Id) {
        self.subscriber.enter(span)
    }

    fn exit(&self, span: &tracing::span::Id) {
        self.subscriber.exit(span)
    }

    fn clone_span(&self, id: &tracing::span::Id) -> tracing::span::Id {
        self.subscriber.clone_span(id)
    }

    #[allow(deprecated)]
    fn drop_span(&self, id: tracing::span::Id) {
        self.subscriber.drop_span(id)
    }
}
