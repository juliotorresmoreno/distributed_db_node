use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ExecutionResult<T> {
    pub result: Option<T>,
    pub error: Option<Box<dyn Error + Send + Sync>>,
}

impl<T> ExecutionResult<T> {
    pub fn success(result: T) -> Self {
        Self {
            result: Some(result),
            error: None,
        }
    }

    pub fn failure<E: Error + Send + Sync + 'static>(err: E) -> Self {
        Self {
            result: None,
            error: Some(Box::new(err)),
        }
    }

    pub fn is_success(&self) -> bool {
        self.result.is_some()
    }
}

#[derive(Debug)]
pub struct TimeoutError;

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "timeout error")
    }
}

impl Error for TimeoutError {}

pub const ERR_TIMEOUT: TimeoutError = TimeoutError;

fn main() {
    let success_result: ExecutionResult<String> = ExecutionResult::success("OK".to_string());
    let error_result: ExecutionResult<()> = ExecutionResult::failure(ERR_TIMEOUT);

    if success_result.is_success() {
        println!("Success: {:?}", success_result.result);
    } else {
        println!("Error: {:?}", success_result.error);
    }

    if error_result.is_success() {
        println!("Success: {:?}", error_result.result);
    } else {
        println!("Error: {:?}", error_result.error);
    }
}
