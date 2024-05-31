use crate::{samples::Randoms, *};

/// This is message that sent to request the creation of a new Randoms struct with the specified id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomsAddRequest(pub u64);

impl IdTargeted for RandomsAddRequest {
    fn id(&self) -> u64 {
        self.0
    }
}

impl RequestWithResponse<Randoms> for RandomsAddRequest {
    type Response = AddResponse;
}

impl From<RandomsAddRequest> for ThreadRequestResponse<Randoms> {
    fn from(add_request: RandomsAddRequest) -> Self {
        ThreadRequestResponse::<Randoms>::AddPoolItem(RequestResponse::Request(add_request))
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
