use crate::id_targeted::IdTargeted;

/// This is the message sent to request that the Randoms struct (with the given id) calculates the mean of the random numbers it contains.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MeanRequest {
    pub id: usize,
}

impl IdTargeted for MeanRequest {
    fn id(&self) -> usize {
        self.id
    }
}
