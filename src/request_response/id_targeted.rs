use crate::id_targeted::IdTargeted;

use super::RequestResponse;

impl<Req, Res> IdTargeted for RequestResponse<Req, Res>
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    fn id(&self) -> usize {
        match self {
            RequestResponse::Request(request) => request.id(),
            RequestResponse::Response(response) => response.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{id_targeted::IdTargeted, request_response::RequestResponse};

    #[test]
    fn request_response_contains_response_of_id_1_returns_1() {
        let target = RequestResponse::<usize, usize>::Response(1);

        assert_eq!(1, target.id());
    }

    #[test]
    fn request_response_contains_response_of_id_2_returns_2() {
        let target = RequestResponse::<usize, usize>::Response(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_response_contains_request_of_id_2_returns_2() {
        let target = RequestResponse::<usize, usize>::Request(2);

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_response_contains_request_of_id_1_returns_1() {
        let target = RequestResponse::<usize, usize>::Request(1);

        assert_eq!(1, target.id());
    }
}
