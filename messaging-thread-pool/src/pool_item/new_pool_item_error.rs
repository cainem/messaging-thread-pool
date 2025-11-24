/// This is a 'better than nothing' error implementation
/// It allows the initialisation code to return error information as a string
#[derive(Debug)]
pub struct NewPoolItemError {
    pub error_message: String,
}
