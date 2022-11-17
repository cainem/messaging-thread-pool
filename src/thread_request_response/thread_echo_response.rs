/// For debug purposes only; a message for responding to an echo request targeting a specific thread
#[derive(Debug, PartialEq, Eq)]
pub struct ThreadEchoResponse {
    thread_id: usize,
    message: String,
    responding_thread_id: usize,
}

impl ThreadEchoResponse {
    pub fn new(thread_id: usize, message: String, responding_thread_id: usize) -> Self {
        Self {
            thread_id,
            message,
            responding_thread_id,
        }
    }

    pub fn thread_id(&self) -> usize {
        self.thread_id
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn responding_thread_id(&self) -> usize {
        self.responding_thread_id
    }
}

// impl IdTargeted for ThreadEchoResponse {
//     fn id(&self) -> u64 {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
