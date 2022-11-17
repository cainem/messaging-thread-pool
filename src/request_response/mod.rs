pub mod id_targeted;

/// This enum is used for defining request/response pairs
///
/// The protocol insists that every request has a corresponding response
/// This enum defines the 2 types that are used
#[derive(Debug, PartialEq, Eq)]
pub enum RequestResponse<Req, Res> {
    Request(Req),
    Response(Res),
}

impl<Req, Res> RequestResponse<Req, Res> {
    pub fn request(&self) -> &Req {
        let RequestResponse::Request(result) = self else {
            panic!("not a request");
        };
        result
    }
    pub fn response(&self) -> &Res {
        let RequestResponse::Response(result) = self else {
            panic!("not a response")
        };
        result
    }
    pub fn is_request(&self) -> bool {
        matches!(self, RequestResponse::Request(_))
    }
    pub fn is_response(&self) -> bool {
        !self.is_request()
    }
}

#[cfg(test)]
mod tests {
    use super::RequestResponse;

    #[test]
    #[should_panic(expected = "not a response")]
    fn request_response_contains_a_request_response_panics() {
        let request = 1;
        let target = RequestResponse::<u64, u64>::Request(request);

        let _ = target.response();
    }

    #[test]
    fn request_response_contains_a_response_response_unwraps_response() {
        let response = 1;
        let target = RequestResponse::<u64, u64>::Response(response);

        assert_eq!(&response, target.response());
    }

    #[test]
    #[should_panic(expected = "not a request")]
    fn request_response_contains_a_response_request_panics() {
        let response = 1;
        let target = RequestResponse::<u64, u64>::Response(response);

        let _ = target.request();
    }

    #[test]
    fn request_response_contains_a_request_request_unwraps_request() {
        let request = 1;
        let target = RequestResponse::<u64, u64>::Request(request);

        assert_eq!(&request, target.request());
    }

    #[test]
    fn request_response_contains_a_response_is_request_returns_false_response_true() {
        let target = RequestResponse::<u64, u64>::Response(1);

        assert!(!target.is_request());
        assert!(target.is_response());
    }

    #[test]
    fn request_response_contains_a_request_is_request_returns_true_response_false() {
        let target = RequestResponse::<u64, u64>::Request(1);

        assert!(target.is_request());
        assert!(!target.is_response());
    }
}
