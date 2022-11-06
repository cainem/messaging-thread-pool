use crate::id_targeted::IdTargeted;

use super::message_processor::MessageProcessor;

/// This trait needs to be implemented by Elements
///
/// It provides an interface for creating a new element
pub trait ElementFactory<Req, Res>: Sized
where
    Req: IdTargeted,
    Res: IdTargeted,
    Self: MessageProcessor<Req, Res>,
{
    fn new_element(request: &Req) -> (Option<Self>, Res);
}
