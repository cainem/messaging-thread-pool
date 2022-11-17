use crate::{
    id_targeted::IdTargeted, request_response::RequestResponse, samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

/// This is message that sent to request the creation of a new Randoms struct with the specified id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomsInitRequest {
    pub id: usize,
}

impl IdTargeted for RandomsInitRequest {
    fn id(&self) -> usize {
        self.id
    }
}

impl From<RandomsInitRequest> for ThreadRequestResponse<Randoms> {
    fn from(init_request: RandomsInitRequest) -> Self {
        ThreadRequestResponse::<Randoms>::AddPoolItem(RequestResponse::Request(init_request))
    }
}
