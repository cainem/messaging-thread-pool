use std::fmt::Debug;

use crate::id_targeted::IdTargeted;

pub trait PoolItemApi: Debug + IdTargeted {
    fn is_request(&self) -> bool;
    fn is_response(&self) -> bool {
        !self.is_request()
    }
}
