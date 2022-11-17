use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadShutdownResponse {
    id: usize,
    children: Vec<ThreadShutdownResponse>,
}

impl ThreadShutdownResponse {
    pub fn new(id: usize, children: Vec<ThreadShutdownResponse>) -> Self {
        Self { id, children }
    }

    pub fn take_children(self) -> Vec<ThreadShutdownResponse> {
        self.children
    }
}

impl IdTargeted for ThreadShutdownResponse {
    fn id(&self) -> usize {
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
