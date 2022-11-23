/// This is a 'better than nothing' error implementation
/// It allows the initialisation code to return error information as a string
#[derive(Debug)]
pub struct NewPoolItemError {
    error_message: String,
}

impl NewPoolItemError {
    pub fn take_error_message(self) -> String {
        self.error_message
    }
}
