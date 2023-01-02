use std::fmt::Debug;

use crate::thread_request_response::ID_ONLY;

/// This trait is implemented by requests and responses to define their relationship to each other \
/// They define a shared constant N and define whether or not they are the response or the request \
/// The implementation of this traits allows for compile time checking of several error conditions
pub trait RequestResponseMessage<const N: usize, const R: bool>: Debug {
    const MESSAGE_TYPE: usize = N;
    const IS_REQUEST: bool = R;
    const IS_RESPONSE: bool = !R;
}

impl RequestResponseMessage<ID_ONLY, true> for usize {}
impl RequestResponseMessage<ID_ONLY, false> for usize {}
