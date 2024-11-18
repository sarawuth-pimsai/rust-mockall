#[derive(Debug)]
pub enum ErrorKind {
    NotImplement,
}
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub code: Option<u32>,
}
impl Error {
    pub fn new(kind: ErrorKind, message: String, code: Option<u32>) -> Self {
        Self {
            kind,
            message,
            code,
        }
    }
}
