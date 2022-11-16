use crate::{
    id_targeted::IdTargeted, request_response_pair::RequestResponse, samples::Randoms,
    thread_request_response::ThreadRequestResponse,
};

/// This is message that sent to request the creation of a new Randoms struct with the specified id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomsInitRequest {
    pub id: u64,
}

impl IdTargeted for RandomsInitRequest {
    fn id(&self) -> u64 {
        self.id
    }
}

impl From<RandomsInitRequest> for ThreadRequestResponse<Randoms> {
    fn from(init_request: RandomsInitRequest) -> Self {
        ThreadRequestResponse::<Randoms>::AddElement(RequestResponse::Request(init_request))
    }
}
