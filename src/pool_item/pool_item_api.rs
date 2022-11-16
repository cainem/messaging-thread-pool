pub trait PoolItemApi {
    fn is_request(&self) -> bool;
    fn is_response(&self) -> bool {
        !self.is_request()
    }
}
