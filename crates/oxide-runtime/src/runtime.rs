use std::fs;
use std::path::{ Path, PathBuf };

use miette::Diagnostic;
use thiserror::Error;
use mlua::{
  Lua,
  chunk::Chunk
};

#[derive(Debug)]
pub struct Runtime {
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
  pub fn new() -> Self {
    let options = RuntimeOptions::default();
    let lua_vm  = Lua::new();

    Runtime { 
      options,
      lua_vm
    }
  }

  fn read_entry_file_content(&self, file_path: &Path) -> Result<String, InitError> {
    fs::read_to_string(file_path)
      .map_err(|error| {
        let path = file_path.to_path_buf();

        InitError::EntryFileReadFailed { path, error }
      })
  }

  pub fn load(&self, file_path: &Path) -> Result<Chunk, RuntimeError> {
    let entry_file_content = self.read_entry_file_content(file_path)?;

    Ok(
      self.lua_vm
        .load(entry_file_content)
        .set_name(file_path.to_string_lossy())
    )
  }

  pub fn run(&mut self, file_path: &Path) -> Result<(), RuntimeError> {
    let chunk = self.load(file_path)?;

    chunk.exec().map_err(ExecutionError::from)?;

    Ok(())
  }
}




