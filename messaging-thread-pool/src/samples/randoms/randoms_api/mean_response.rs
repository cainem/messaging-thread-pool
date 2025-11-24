/// The response from a request to calculate the mean
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeanResponse {
    pub id: u64,
    pub mean: u128,
}

impl MeanResponse {
    pub fn mean(&self) -> u128 {
        self.mean
    }
}
