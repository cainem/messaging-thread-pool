use crate::id_targeted::IdTargeted;

/// The response from a request to create a new Randoms struct within the thread pool
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomsInitResponse {
    pub id: usize,
}

impl IdTargeted for RandomsInitResponse {
    fn id(&self) -> usize {
        self.id
    }
}
