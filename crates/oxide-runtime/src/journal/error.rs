use thiserror::Error;
use miette::Diagnostic;

#[derive(Debug, Error, Diagnostic, PartialEq, Eq)]
pub enum JournalError {
  #[error(transparent)]
  #[diagnostic(transparent)]
  Operational(#[from] JournalOperationalError),

  #[error(transparent)]
  #[diagnostic(transparent)]
  Internal(#[from] JournalInternalError)
}

#[derive(Debug, Error, Diagnostic, PartialEq, Eq)]
pub enum JournalOperationalError {
  #[error("journal capacity must be greater than 0")]
  #[diagnostic(
    code(oxide::journal::zero_capacity),
    help("ensure capacity passed to Journal::new or set_capacity is >= 1")
  )]
  ZeroCapacity
}

#[derive(Debug, Error, Diagnostic, PartialEq, Eq)]
pub enum JournalInternalError {
  #[error("internal state of the journal has been corrupted")]
  #[diagnostic(
    code(oxide::journal::state_corrupted),
    help("this is a bug in oxide runtime invariants, please report it")
  )]
  StateCorrupted
}
