use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct PostgresDetailedError {
    pub code: String,
    pub message: String,
    pub detail: Option<String>,
    pub hint: Option<String>,
    pub schema: Option<String>,
    pub table: Option<String>,
    pub column: Option<String>,
    pub datatype: Option<String>,
    pub constraint: Option<String>,
    pub where_context: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub routine: Option<String>,
}

impl fmt::Display for PostgresDetailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Postgres Error [{}]: {}", self.code, self.message)?;
        if let Some(detail) = &self.detail {
            write!(f, "\nDetail: {}", detail)?;
        }
        if let Some(hint) = &self.hint {
            write!(f, "\nHint: {}", hint)?;
        }
        if let Some(schema) = &self.schema {
            write!(f, "\nSchema: {}", schema)?;
        }
        if let Some(table) = &self.table {
            write!(f, "\nTable: {}", table)?;
        }
        if let Some(column) = &self.column {
            write!(f, "\nColumn: {}", column)?;
        }
        if let Some(constraint) = &self.constraint {
            write!(f, "\nConstraint: {}", constraint)?;
        }
        if let Some(where_context) = &self.where_context {
            write!(f, "\nWhere: {}", where_context)?;
        }
        if let Some(file) = &self.file {
            write!(f, "\nFile: {}:{}", file, self.line.unwrap_or(0))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConnectorError {
    ConnectionError(bb8_postgres::tokio_postgres::Error),
    PostgresError(PostgresDetailedError),
    PoolError(String),
    TimeoutError,
}

impl fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectorError::ConnectionError(e) => write!(f, "Postgres connection error: {}", e),
            ConnectorError::PostgresError(e) => write!(f, "{}", e),
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
        if let Some(db_err) = err.as_db_error() {
            ConnectorError::PostgresError(PostgresDetailedError {
                code: db_err.code().code().to_string(),
                message: db_err.message().to_string(),
                detail: db_err.detail().map(|s| s.to_string()),
                hint: db_err.hint().map(|s| s.to_string()),
                schema: db_err.schema().map(|s| s.to_string()),
                table: db_err.table().map(|s| s.to_string()),
                column: db_err.column().map(|s| s.to_string()),
                datatype: db_err.datatype().map(|s| s.to_string()),
                constraint: db_err.constraint().map(|s| s.to_string()),
                where_context: db_err.where_().map(|s| s.to_string()),
                file: db_err.file().map(|s| s.to_string()),
                line: db_err.line(),
                routine: db_err.routine().map(|s| s.to_string()),
            })
        } else {
            ConnectorError::ConnectionError(err)
        }
    }
}

impl From<bb8::RunError<bb8_postgres::tokio_postgres::Error>> for ConnectorError {
    fn from(err: bb8::RunError<bb8_postgres::tokio_postgres::Error>) -> Self {
        match err {
            bb8::RunError::User(e) => ConnectorError::from(e),
            bb8::RunError::TimedOut => ConnectorError::TimeoutError,
        }
    }
}
