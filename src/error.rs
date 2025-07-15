#[derive(Debug)]
pub enum SpadeError {
    RuntimeError { message: String, line: usize },
    Return(crate::evaluate::Value),
}

impl SpadeError {
    pub fn runtime_error(message: String, line: usize) -> Self {
        SpadeError::RuntimeError { message, line }
    }
    
    pub fn return_value(value: crate::evaluate::Value) -> Self {
        SpadeError::Return(value)
    }
}
