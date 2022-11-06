use crate::{id_targeted::IdTargeted, thread_shutdown_response::ThreadShutdownResponse};

/// This enum represents a response returned from a PoolThread
///
/// There are a 4 thread management variants ThreadAbort, ThreadShutdown, EchoThread, and RemoveElement
/// and 1 variant for handling actual responses from elements within the PoolThread (ElementResponse)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThreadResponse<Res>
where
    Res: IdTargeted,
{
    ThreadShutdown(ThreadShutdownResponse),
    ThreadAbort(u64),
    ThreadEcho(u64, u64, String),
    RemoveElement(u64),
    ElementResponse(Res),
}

impl<Res> IdTargeted for ThreadResponse<Res>
where
    Res: IdTargeted,
{
    fn get_id(&self) -> u64 {
        match self {
            ThreadResponse::ThreadAbort(id) => *id,
            ThreadResponse::ThreadShutdown(thread_shutdown_payload) => thread_shutdown_payload.id(),
            ThreadResponse::ThreadEcho(id, _, _) => *id,
            ThreadResponse::RemoveElement(id) => *id,
            ThreadResponse::ElementResponse(request) => request.get_id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted,
        samples::randoms::randoms_response::{
            init_response::InitResponse, sum_response::SumResponse, RandomsResponse,
        },
        thread_shutdown_response::ThreadShutdownResponse,
    };

    use super::ThreadResponse;

    #[test]
    fn thread_abort_targets_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ThreadAbort(2);

        assert_eq!(2, target.get_id());
    }

    #[test]
    fn thread_abort_targets_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ThreadAbort(1);

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn element_response_from_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ElementResponse(RandomsResponse::Init(
            InitResponse { id: 2 },
        ));

        assert_eq!(2, target.get_id());
    }

    #[test]
    fn element_response_from_id_1_get_id_returns_1() {
        let target =
            ThreadResponse::<RandomsResponse>::ElementResponse(RandomsResponse::Sum(SumResponse {
                id: 1,
                sum: 5,
            }));

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn remove_element_from_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::RemoveElement(2);

        assert_eq!(2, target.get_id());
    }

    #[test]
    fn remove_element_from_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::RemoveElement(1);

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn thread_echo_from_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ThreadEcho(2, 0, "ping".to_string());

        assert_eq!(2, target.get_id());
    }

    #[test]
    fn thread_echo_from_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ThreadEcho(1, 0, "ping".to_string());

        assert_eq!(1, target.get_id());
    }

    #[test]
    fn thread_shutdown_targets_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ThreadShutdown(
            ThreadShutdownResponse::new(2, vec![]),
        );

        assert_eq!(2, target.get_id());
    }

    #[test]
    fn thread_shutdown_targets_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ThreadShutdown(
            ThreadShutdownResponse::new(1, vec![]),
        );

        assert_eq!(1, target.get_id());
    }
}
