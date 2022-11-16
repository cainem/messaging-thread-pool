use crate::id_targeted::IdTargeted;

/// This is the message sent to request that the Randoms struct (with the given id) calculates the mean of the random numbers it contains.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MeanRequest {
    pub id: u64,
}

impl IdTargeted for MeanRequest {
    fn id(&self) -> u64 {
        self.id
    }
}
