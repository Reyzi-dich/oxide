use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum RuntimeError {
  #[error("runtime initialization failed")]
  #[diagnostic(code(oxide::runtime::init_failed))]
  Init(#[from] #[source] InitError),

  #[error("runtime execution failed")]
  #[diagnostic(code(oxide::runtime::execution_failed))]
  Execution(#[from] #[source] ExecutionError),
}

#[derive(Debug, Error, Diagnostic)]
pub enum InitError {
  #[error("failed to read entry file '{path}'")]
  #[diagnostic(
    code(oxide::runtime::init::entry_file_read_failed),
    help("check if the path exists and oxide has read permissions")
  )]
  EntryFileReadFailed {
    path: PathBuf,
    #[source]
    error: std::io::Error,
  },

  #[error("failed to initialize Luau VM")]
  #[diagnostic(code(oxide::runtime::init::luau_init_failed))]
  LuaInitFailed(#[from] #[source] mlua::Error),
}

#[derive(Debug, Error, Diagnostic)]
pub enum ExecutionError {
  #[error("script execution failed")]
  #[diagnostic(code(oxide::runtime::execution::script_failed))]
  LuaScriptFailed(#[from] #[source] mlua::Error),
}
