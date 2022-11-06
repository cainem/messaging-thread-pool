#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadShutdownResponse {
    id: u64,
    children: Vec<ThreadShutdownResponse>,
}

impl ThreadShutdownResponse {
    pub fn new(id: u64, children: Vec<ThreadShutdownResponse>) -> Self {
        Self { id, children }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn children(&self) -> &[ThreadShutdownResponse] {
        self.children.as_ref()
    }

    pub fn take_children(self) -> Vec<ThreadShutdownResponse> {
        self.children
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadShutdownResponse;

    #[test]
    fn children_non_empty_take_takes_vec() {
        let children = vec![ThreadShutdownResponse::new(10, vec![])];
        let target = ThreadShutdownResponse::new(1, children.clone());

        assert_eq!(children, target.take_children());
    }

    #[test]
    fn children_empty_take_takes_empty_vec() {
        let target = ThreadShutdownResponse::new(1, vec![]);

        assert_eq!(
            Vec::<ThreadShutdownResponse>::default(),
            target.take_children()
        );
    }
}
