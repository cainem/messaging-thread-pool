use crate::ID_BEING_PROCESSED;

/// This function can be used externally to access the id of the pool item that is currently
/// being processed
pub fn id_being_processed() -> Option<usize> {
    ID_BEING_PROCESSED.with(|id_being_processed| *id_being_processed.borrow())
}
