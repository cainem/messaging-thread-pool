pub mod add_response;
pub mod remove_response;
pub mod thread_echo;
pub mod thread_shutdown_response;

use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response_pair::RequestResponse};

use self::{
    add_response::AddResponse,
    remove_response::RemoveResponse,
    thread_echo::{ThreadEchoRequest, ThreadEchoResponse},
    thread_shutdown_response::ThreadShutdownResponse,
};

#[derive(Debug, PartialEq)]
pub enum ThreadRequestResponse<P>
where
    P: PoolItem,
{
    ThreadShutdown(RequestResponse<u64, ThreadShutdownResponse>),
    ThreadAbort(RequestResponse<u64, u64>),
    ThreadEcho(RequestResponse<ThreadEchoRequest, ThreadEchoResponse>),
    RemoveElement(RequestResponse<u64, RemoveResponse>),
    AddElement(RequestResponse<P::Init, AddResponse>),
    CallElement(P::Api),
}

impl<P> ThreadRequestResponse<P>
where
    P: PoolItem,
{
    pub fn is_request(&self) -> bool {
        match self {
            ThreadRequestResponse::ThreadShutdown(payload) => payload.is_request(),
            ThreadRequestResponse::ThreadAbort(_) => todo!(),
            ThreadRequestResponse::ThreadEcho(_) => todo!(),
            ThreadRequestResponse::RemoveElement(_) => todo!(),
            ThreadRequestResponse::AddElement(_) => todo!(),
            ThreadRequestResponse::CallElement(payload) => todo!(),
        }
    }

    pub fn is_response(&self) -> bool {
        !self.is_request()
    }
}

impl<P> IdTargeted for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn id(&self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
