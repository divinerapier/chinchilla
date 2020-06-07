#[derive(Debug)]
pub enum Error {
    R2D2(r2d2::Error),
    MySQL(mysql::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::R2D2(ref e) => write!(f, "r2d2 error: {}", e),
            Error::MySQL(ref e) => write!(f, "mysql error: {}", e),
            _ => unreachable!(),
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::R2D2(e)
    }
}

impl From<mysql::Error> for Error {
    fn from(e: mysql::Error) -> Error {
        Error::MySQL(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

