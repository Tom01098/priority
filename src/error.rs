use diesel::ConnectionError;
use diesel::result::Error as DieselError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Database(#[from] DatabaseError),

    #[error("{0}")]
    Environment(#[from] EnvironmentError),
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::Database(err) => err.exit_code(),
            Error::Environment(err) => err.exit_code(),
        }
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database at {url}: {source}")]
    Connection {
        url: String,
        source: ConnectionError,
    },
    #[error("Failed to run database migrations at {url}: {source}")]
    Migration {
        url: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("Database query failed: {source}")]
    Query { source: DieselError },
}

impl DatabaseError {
    pub fn exit_code(&self) -> i32 {
        match self {
            DatabaseError::Connection { .. } => 2,
            DatabaseError::Migration { .. } => 2,
            DatabaseError::Query { .. } => 3,
        }
    }
}

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("Failed to determine home directory")]
    HomeDir,
}

impl EnvironmentError {
    pub fn exit_code(&self) -> i32 {
        2
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        Error::Database(DatabaseError::Query { source: err })
    }
}
