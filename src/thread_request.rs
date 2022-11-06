use crate::id_targeted::IdTargeted;

/// This enum represent a request sent to a PoolThread.
///
/// It has three variants that communicate to the PoolThread itself (ThreadShutdown, ThreadEcho and RemoveElement)
/// and one variant that deals with requests to the elements stored within the PoolThread
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThreadRequest<Req>
where
    Req: IdTargeted,
{
    ThreadShutdown(u64),
    ThreadAbort(u64),
    ThreadEcho(u64, String),
    RemoveElement(u64),
    ElementRequest(Req),
}

impl<Req> IdTargeted for ThreadRequest<Req>
where
    Req: IdTargeted,
{
    fn get_id(&self) -> u64 {
        match self {
            ThreadRequest::ThreadShutdown(id) => *id,
            ThreadRequest::ThreadAbort(id) => *id,
            ThreadRequest::ThreadEcho(id, _) => *id,
            ThreadRequest::RemoveElement(id) => *id,
            ThreadRequest::ElementRequest(request) => request.get_id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted,
        samples::randoms::randoms_request::{
            mean_request::MeanRequest, sum_request::SumRequest, RandomsRequest,
        },
        thread_request::ThreadRequest,
    };

    #[test]
    fn thread_abort_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ThreadAbort(1);

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn thread_abort_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ThreadAbort(0);

        assert_eq!(0, target.get_id());
    }

    #[test]
    fn element_request_id_1_returns_1() {
        let target =
            ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Mean(MeanRequest {
                id: 1,
            }));

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn element_request_id_0_returns_0() {
        let target =
            ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Sum(SumRequest {
                id: 0,
            }));

        assert_eq!(0, target.get_id());
    }

    #[test]
    fn remove_element_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::RemoveElement(1);

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn remove_element_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::RemoveElement(0);

        assert_eq!(0, target.get_id());
    }

    #[test]
    fn thread_echo_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ThreadEcho(1, "ping".to_string());

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn thread_echo_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ThreadEcho(0, "ping".to_string());

        assert_eq!(0, target.get_id());
    }

    #[test]
    fn thread_shutdown_id_1_returns_1() {
        let target = ThreadRequest::<RandomsRequest>::ThreadShutdown(1);

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn thread_shutdown_id_0_returns_0() {
        let target = ThreadRequest::<RandomsRequest>::ThreadShutdown(0);

        assert_eq!(0, target.get_id());
    }
}
