use crate::IdTargeted;

/// This is message that sent to request the creation of a new Randoms struct with the specified id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomsAddRequest(pub u64);

impl IdTargeted for RandomsAddRequest {
    fn id(&self) -> u64 {
        self.0
    }
}
