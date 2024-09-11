use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_yaml::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
