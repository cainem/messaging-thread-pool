use crate::{samples::Randoms, *};

use super::RandomsApi;

/// The response from a request to calculate the mean
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicResponse(pub u64);

// tie the response to an api call
bind_response_to_api!(PanicResponse, Randoms, RandomsApi::Panic);
