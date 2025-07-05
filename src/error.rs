use std::{error::Error, fmt};

#[derive(Debug)]
pub enum GetFileError {
    MultipleFileError(String),
    CopyFileError(),
    DirectoryError(String),
}
impl fmt::Display for GetFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GetFileError::MultipleFileError(extension) => write!(f, "There are multiple files. There should be only one lmt file and one ldb file.: {extension}"),
            GetFileError::DirectoryError(path) => write!(f, "Directory does not exist.: {path}"),
            GetFileError::CopyFileError() => write!(f, "Failed to copy file.: "),
        }
    }
}
impl Error for GetFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}