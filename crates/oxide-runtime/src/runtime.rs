use std::io;
use std::path::PathBuf;

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

#[derive(Debug, thiserror::Error)]
pub enum InitError {
  #[error("Failed to read entry file '{path}': {error}")]
  EntryFileReadFailed { path: PathBuf, error: io::Error },

  #[error("Failed to initialize Luau VM: {0}")]
  LuaInitFailed(#[from] mlua::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
  #[error("Script execution failed: {0}")]
  LuaScriptFailed(#[from] mlua::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
  #[error("Initialization error: {0}")]
  Init(#[from] InitError),

  #[error("Execution error: {0}")]
  Execution(#[from] ExecutionError),
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




