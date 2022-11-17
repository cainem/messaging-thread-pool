use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse, samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

/// This is message that sent to request the creation of a new Randoms struct with the specified id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomsAddRequest(pub usize);

impl IdTargeted for RandomsAddRequest {
    fn id(&self) -> usize {
        self.0
    }
}

impl From<RandomsAddRequest> for ThreadRequestResponse<Randoms> {
    fn from(init_request: RandomsAddRequest) -> Self {
        ThreadRequestResponse::<Randoms>::AddPoolItem(RequestResponse::Request(init_request))
    }
}

impl From<ThreadRequestResponse<Randoms>> for RandomsAddRequest {
    fn from(response: ThreadRequestResponse<Randoms>) -> Self {
        let ThreadRequestResponse::AddPoolItem(RequestResponse::Request(result)) = response else {
            panic!("not expected")
        };
        result
    }
}
