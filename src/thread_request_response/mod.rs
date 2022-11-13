pub mod add_response;
pub mod remove_response;
pub mod thread_echo;
pub mod thread_shutdown_response;

use crate::{element::request_response_pair::RequestResponse, pool_item::PoolItem};

use self::{
    add_response::AddResponse,
    remove_response::RemoveResponse,
    thread_echo::{ThreadEchoRequest, ThreadEchoResponse},
    thread_shutdown_response::ThreadShutdownResponse,
};

#[derive(Debug)]
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
}
