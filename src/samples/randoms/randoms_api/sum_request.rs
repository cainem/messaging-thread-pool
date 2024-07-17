use crate::IdTargeted;

#[derive(Debug, PartialEq, Eq)]
pub struct SumRequest(pub u64);

impl IdTargeted for SumRequest {
    fn id(&self) -> u64 {
        self.0
    }
}
