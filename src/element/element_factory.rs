use crate::id_targeted::IdTargeted;

use super::message_processor::MessageProcessor;

/// This trait needs to be implemented by Elements.\
/// It provides an interface for creating a new instance of the element.
pub trait ElementFactory<Req, Res>: Sized
where
    Req: IdTargeted,
    Res: IdTargeted,
    Self: MessageProcessor<Req, Res>,
{
    /// This function is implemented to define how a new instance of the implementer is created
    /// by the thread pool infrastructure
    fn new_element(request: &Req) -> (Option<Self>, Res);
}
