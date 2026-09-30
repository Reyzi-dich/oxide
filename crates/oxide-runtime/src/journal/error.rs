use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum JournalError {
  #[error("Operational: {0}")]
  Operational(#[from] JournalOperationalError),

  #[error("Internal: {0}")]
  Internal(#[from] JournalInternalError)
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum JournalOperationalError {
  #[error("Journal capacity must be greater than 0")]
  ZeroCapacity
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum JournalInternalError {
  #[error("Internal State of the `journal` has been corrupted")]
  StateCorrupted
}
