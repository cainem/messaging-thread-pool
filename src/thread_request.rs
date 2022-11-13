use crate::id_targeted::IdTargeted;

/// This enum represent a request sent to a thread pool.
///
/// It has three variants that communicate to the thread pool itself (ThreadShutdown, ThreadEcho and RemoveElement)
/// and one variant (ElementRequest) that deals with requests to the elements stored within the thread pool
///
/// ElementRequest is in essence the variant were the custom api of the given Element is bundled.
///
/// All other variants are to managed the thread pool itself (or to help in debugging the thread pool)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThreadRequest<Req>
where
    Req: IdTargeted,
{
    /// Request that a thread with the given id is shutdown.\
    /// This remove all of the elements from the threads internal storage and calls
    /// shutdown on the contained elements giving them the opportunity to shutdown any of their internal
    /// thread pool.\
    /// The thread is joined back to the main thread.\
    ///
    /// NOTE: there is an assumption here that all threads will be requested to be shutdown together.
    /// There is currently no supported mechanism for dynamically sizing the thread pool.
    ThreadShutdown(u64),
    /// Request that a thread with the given id is shutdown.\
    /// This joins the thread back to the main thread but doesn't call shutdown on any of its contained elements
    /// and it doesn't remove the elements from its internal state.\
    ///
    /// This is used for testing.
    ThreadAbort(u64),
    /// For testing purposes a message that just echoes back the message sent to it with an indication of which thread
    /// processed it.
    ThreadEcho(u64, String),
    /// This drops an element from the thread pools internal storage.
    RemoveElement(u64),
    /// This is the variant that holds the messages that are targeted at the element itself.
    /// Req here will be another enum whose variants define the elements api.
    ElementRequest(Req),
}

impl<Req> IdTargeted for ThreadRequest<Req>
where
    Req: IdTargeted,
{
    fn id(&self) -> u64 {
        match self {
            ThreadRequest::ThreadShutdown(id) => *id,
            ThreadRequest::ThreadAbort(id) => *id,
            ThreadRequest::ThreadEcho(id, _) => *id,
            ThreadRequest::RemoveElement(id) => *id,
            ThreadRequest::ElementRequest(request) => request.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{id_targeted::IdTargeted, samples::*, thread_request::ThreadRequest};

    #[test]
    fn thread_abort_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ThreadAbort(1);

        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_abort_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ThreadAbort(0);

        assert_eq!(0, target.id());
    }

    #[test]
    fn element_request_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Mean(
            mean_request::MeanRequest { id: 1 },
        ));

        assert_eq!(1, target.id());
    }

    #[test]
    fn element_request_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Sum(
            sum_request::SumRequest { id: 0 },
        ));

        assert_eq!(0, target.id());
    }

    #[test]
    fn remove_element_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::RemoveElement(1);

        assert_eq!(1, target.id());
    }

    #[test]
    fn remove_element_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::RemoveElement(0);

        assert_eq!(0, target.id());
    }

    #[test]
    fn thread_echo_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ThreadEcho(1, "ping".to_string());

        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_echo_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ThreadEcho(0, "ping".to_string());

        assert_eq!(0, target.id());
    }

    #[test]
    fn thread_shutdown_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ThreadShutdown(1);

        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_shutdown_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ThreadShutdown(0);

        assert_eq!(0, target.id());
    }
}
