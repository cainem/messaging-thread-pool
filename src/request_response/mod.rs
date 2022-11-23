use self::request_response_message::RequestResponseMessage;

mod id_targeted;
pub mod request_message;
pub mod request_response_message;
pub mod response_message;

/// This enum is used for defining request/response pairs \
/// The protocol insists that every request has a corresponding response \
/// This enum defines the 2 types that are used
#[derive(Debug, PartialEq, Eq)]
pub enum RequestResponse<const N: usize, Req, Res>
where
    Req: RequestResponseMessage<N, true>,
    Res: RequestResponseMessage<N, false>,
{
    Request(Req),
    Response(Res),
}

impl<const N: usize, Req, Res> RequestResponse<N, Req, Res>
where
    Req: RequestResponseMessage<N, true>,
    Res: RequestResponseMessage<N, false>,
{
    pub fn request(&self) -> &Req {
        let RequestResponse::Request(result) = self else {
            panic!("not a request");
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::thread_request_response::ID_ONLY;

    use super::RequestResponse;

    #[test]
    #[should_panic(expected = "not a request")]
    fn request_response_contains_a_response_request_panics() {
        let response = 1;
        let target = RequestResponse::<ID_ONLY, usize, usize>::Response(response);

        let _ = target.request();
    }

    #[test]
    fn request_response_contains_a_request_request_unwraps_request() {
        let request = 1;
        let target = RequestResponse::<ID_ONLY, usize, usize>::Request(request);

        assert_eq!(&request, target.request());
    }
}
