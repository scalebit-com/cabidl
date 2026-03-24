use std::fmt;

/// A validation error with location context.
///
/// Formats as `file:line: message` when a line number is present,
/// or `file: message` when it is not — following compiler diagnostic conventions.
#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}:{}: {}", self.file, line, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}
