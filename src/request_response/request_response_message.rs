use crate::{id_targeted::IdTargeted, thread_request_response::ID_ONLY};

pub trait RequestResponseMessage<const N: usize, const R: bool>: IdTargeted {
    const MESSAGE_TYPE: usize = N;
    const IS_REQUEST: bool = R;
    const IS_RESPONSE: bool = !R;
}

impl RequestResponseMessage<ID_ONLY, true> for usize {}
impl RequestResponseMessage<ID_ONLY, false> for usize {}
