use std::error::Error;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExecutionResult<T> {
    pub result: Option<T>,
    pub error: Option<Box<dyn Error + Send + Sync>>,
}

#[allow(dead_code)]
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
