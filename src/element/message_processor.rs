use crate::id_targeted::IdTargeted;

/// This trait needs to be implemented by Elements.\
/// This provides a function that is in essence the interface supported by the element.\
/// It takes a request and returns a response.\
/// In practice both the requests and responses will be split into message types by the use of
/// enums with the different variants defining the elements interface.\
/// The primary function of the process_message function will therefore be to handle these different
/// request/response variants.
pub trait MessageProcessor<Req, Res>: Sized
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    /// Adds debug assertions around the implementation of the process message implementation.
    ///
    /// This does not need to be implemented. The default implementation
    fn process_message_checked(&mut self, request: &Req) -> Res {
        if cfg!(debug_assertions) {
            // check that the response matches the request
            let targeted_id = request.id();
            let response = self.process_message(request);
            assert_eq!(
                targeted_id,
                response.id(),
                "the request and response ids should always match"
            );

            response
        } else {
            self.process_message(request)
        }
    }

    /// This is the function that handles all of the messages that are aimed at the element contained
    /// within the thread pool
    /// It is this functions responsibility to act upon the messages sent
    fn process_message(&mut self, request: &Req) -> Res;
}
