use crate::{
    element::request_response_pair::RequestResponse, id_targeted::IdTargeted, pool_item::PoolItem,
};

use super::ThreadRequestResponse;

#[derive(Debug)]
pub struct AddResponse {
    id: u64,
    success: bool,
}

impl AddResponse {
    pub fn new(id: u64, success: bool) -> Self {
        Self { id, success }
    }
}

impl IdTargeted for AddResponse {
    fn id(&self) -> u64 {
        todo!()
    }
}

impl<T> From<AddResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: AddResponse) -> Self {
        ThreadRequestResponse::AddElement(RequestResponse::Response(request))
    }
}

// impl From<ThreadRequestResponse<Randoms>> for MeanResponse {
//     fn from(response: ThreadRequestResponse<Randoms>) -> Self {
//         let ThreadRequestResponse::CallElement(RandomsApi::Mean(
//             RequestResponse::Response(result))) = response else {
//                 panic!("must be a response to a call to the element")
//             };
//         result
//     }
// }
