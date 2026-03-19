use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ConnectorError {
    ConnectionError(bb8_postgres::tokio_postgres::Error),
    PoolError(String),
    TimeoutError,
}

impl fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectorError::ConnectionError(e) => write!(f, "Postgres connection error: {}", e),
            ConnectorError::PoolError(e) => write!(f, "BB8 pool error: {}", e),
            ConnectorError::TimeoutError => write!(f, "Operation timed out"),
        }
    }
}

impl Error for ConnectorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConnectorError::ConnectionError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<bb8_postgres::tokio_postgres::Error> for ConnectorError {
    fn from(err: bb8_postgres::tokio_postgres::Error) -> Self {
        ConnectorError::ConnectionError(err)
    }
}

impl From<bb8::RunError<bb8_postgres::tokio_postgres::Error>> for ConnectorError {
    fn from(err: bb8::RunError<bb8_postgres::tokio_postgres::Error>) -> Self {
        match err {
            bb8::RunError::User(e) => ConnectorError::ConnectionError(e),
            bb8::RunError::TimedOut => ConnectorError::TimeoutError,
        }
    }
}
