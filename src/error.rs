use std::fmt::Display;
use crate::command::CommandError;
use crate::store::StoreError;

#[derive(Debug)]
pub enum AppError {
    Store(StoreError),
    Command(CommandError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Store(e) => write!(f, "Store error: {:?}", e),
            AppError::Command(e) => write!(f, "Command error: {:?}", e),
        }
    }
}

impl From<StoreError> for AppError {
    fn from(e: StoreError) -> Self {
        AppError::Store(e)
    }
}

impl From<CommandError> for AppError {
    fn from(e: CommandError) -> Self {
        AppError::Command(e)
    }
}