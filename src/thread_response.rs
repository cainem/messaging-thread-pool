use crate::id_targeted::IdTargeted;

/// This enum represents a response returned from a PoolThread
///
/// There are a 4 thread management variants ThreadAbort, ThreadShutdown, EchoThread, and RemoveElement
/// and 1 variant for handling actual responses from elements within the PoolThread (ElementResponse).
///
/// See [`super::thread_request::ThreadRequest`] for more detailed explanation of the individual messages.
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
    fn id(&self) -> u64 {
        match self {
            ThreadResponse::ThreadAbort(id) => *id,
            ThreadResponse::ThreadShutdown(thread_shutdown_payload) => thread_shutdown_payload.id(),
            ThreadResponse::ThreadEcho(id, _, _) => *id,
            ThreadResponse::RemoveElement(id) => *id,
            ThreadResponse::ElementResponse(request) => request.id(),
        }
    }
}

/// This struct represents the information returned from a shutdown request.\
/// It contains the id of the shutdown thread and potentially a vec of shutdown
/// responses from any child threads. This vec will be empty if there are no child threads.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadShutdownResponse {
    id: u64,
    children: Vec<ThreadShutdownResponse>,
}

impl ThreadShutdownResponse {
    pub fn new(id: u64, children: Vec<ThreadShutdownResponse>) -> Self {
        Self { id, children }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn children(&self) -> &[ThreadShutdownResponse] {
        self.children.as_ref()
    }

    pub fn take_children(self) -> Vec<ThreadShutdownResponse> {
        self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::{id_targeted::IdTargeted, samples::*, thread_response::ThreadShutdownResponse};

    use super::ThreadResponse;

    #[test]
    fn children_non_empty_take_takes_vec() {
        let children = vec![ThreadShutdownResponse::new(10, vec![])];
        let target = ThreadShutdownResponse::new(1, children.clone());

        assert_eq!(children, target.take_children());
    }

    #[test]
    fn children_empty_take_takes_empty_vec() {
        let target = ThreadShutdownResponse::new(1, vec![]);

        assert_eq!(
            Vec::<ThreadShutdownResponse>::default(),
            target.take_children()
        );
    }

    #[test]
    fn thread_abort_targets_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ThreadAbort(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn thread_abort_targets_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ThreadAbort(1);

        assert_eq!(1, target.id());
    }

    #[test]
    fn element_response_from_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ElementResponse(RandomsResponse::Init(
            randoms_init_response::RandomsInitResponse { id: 2 },
        ));

        assert_eq!(2, target.id());
    }

    #[test]
    fn element_response_from_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ElementResponse(RandomsResponse::Sum(
            sum_response::SumResponse { id: 1, sum: 5 },
        ));

        assert_eq!(1, target.id());
    }

    #[test]
    fn remove_element_from_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::RemoveElement(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn remove_element_from_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::RemoveElement(1);

        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_echo_from_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ThreadEcho(2, 0, "ping".to_string());

        assert_eq!(2, target.id());
    }

    #[test]
    fn thread_echo_from_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ThreadEcho(1, 0, "ping".to_string());

        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_shutdown_targets_id_2_get_id_returns_2() {
        let target = ThreadResponse::<RandomsResponse>::ThreadShutdown(
            ThreadShutdownResponse::new(2, vec![]),
        );

        assert_eq!(2, target.id());
    }

    #[test]
    fn thread_shutdown_targets_id_1_get_id_returns_1() {
        let target = ThreadResponse::<RandomsResponse>::ThreadShutdown(
            ThreadShutdownResponse::new(1, vec![]),
        );

        assert_eq!(1, target.id());
    }
}
