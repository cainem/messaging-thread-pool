use crate::id_targeted::IdTargeted;

#[derive(Debug)]
pub struct ThreadEchoRequest {}

#[derive(Debug)]
pub struct ThreadEchoResponse {}

impl IdTargeted for ThreadEchoRequest {
    fn id(&self) -> u64 {
        todo!()
    }
}

impl IdTargeted for ThreadEchoResponse {
    fn id(&self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
