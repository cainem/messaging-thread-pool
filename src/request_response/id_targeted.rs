use crate::id_targeted::IdTargeted;

use super::{request_response_message::RequestResponseMessage, RequestResponse};

impl<const N: usize, Req, Res> IdTargeted for RequestResponse<N, Req, Res>
where
    Req: RequestResponseMessage<N, true> + IdTargeted,
    Res: RequestResponseMessage<N, false>,
{
    fn id(&self) -> usize {
        match self {
            RequestResponse::Request(request) => request.id(),
            RequestResponse::Response(_response) => {
                panic!("id targeting is not required for responses")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted, request_response::RequestResponse,
        thread_request_response::ID_ONLY,
    };

    #[test]
    fn request_response_contains_request_of_id_2_returns_2() {
        let target = RequestResponse::<ID_ONLY, usize, usize>::Request(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_response_contains_request_of_id_1_returns_1() {
        let target = RequestResponse::<ID_ONLY, usize, usize>::Request(1);

        assert_eq!(1, target.id());
    }
}
