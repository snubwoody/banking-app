use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Failed to run migration: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}
