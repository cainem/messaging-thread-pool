/// For debug purposes only send a message to a thread within the thread pool
#[derive(Debug, PartialEq, Eq)]
pub struct ThreadEchoRequest {
    thread_id: usize,
    message: String,
}

impl ThreadEchoRequest {
    pub fn new(thread_id: usize, message: String) -> Self {
        Self { thread_id, message }
    }

    pub fn thread_id(&self) -> usize {
        self.thread_id
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
