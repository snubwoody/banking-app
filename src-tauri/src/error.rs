use std::io;


#[derive(Debug,thiserror::Error)]
pub enum Error{
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] io::Error)
}