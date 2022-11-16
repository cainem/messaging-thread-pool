use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response_pair::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadShutdownResponse {
    id: u64,
    children: Vec<ThreadShutdownResponse>,
}

impl ThreadShutdownResponse {
    pub fn new(id: u64, children: Vec<ThreadShutdownResponse>) -> Self {
        Self { id, children }
    }

    pub fn take_children(self) -> Vec<ThreadShutdownResponse> {
        self.children
    }
}

impl IdTargeted for ThreadShutdownResponse {
    fn id(&self) -> u64 {
        todo!()
    }
}

impl<T> From<ThreadShutdownResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: ThreadShutdownResponse) -> Self {
        ThreadRequestResponse::ThreadShutdown(RequestResponse::Response(request))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
