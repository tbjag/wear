#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    ReError(regex::Error)
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IOError(e)
    }
}

impl From<regex::Error> for AppError {
    fn from(e: regex::Error) -> Self {
        AppError::ReError(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IOError(err) => write!(f, "IO Error: {}", err),
            AppError::ReError(err) => write!(f, "Regex Error: {}", err),
        }
    }
}
