use std::io;
use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;
use mlua::Lua;

#[derive(Debug)]
pub struct Runtime {
  entry_path: PathBuf,
  options:    RuntimeOptions,
  lua_vm:     Lua
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeOptions {
  debug: bool,
  trace: bool
}

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

impl Runtime {
  pub fn new(entry_path: PathBuf) -> Self {
    let options = RuntimeOptions::default();
    let lua_vm  = Lua::new();

    Runtime { 
      entry_path,
      options,
      lua_vm
    }
  }

  fn read_entry_file_content(&self) -> Result<String, InitError> {
    std::fs::read_to_string(&self.entry_path)
      .map_err(|error| {
        let path = self.entry_path.clone();

        InitError::EntryFileReadFailed { path, error }
      })
  }

  pub fn run(&mut self) -> Result<(), RuntimeError> {
    let entry_file_content = self.read_entry_file_content()?;

    

    Ok(())
  }
}




