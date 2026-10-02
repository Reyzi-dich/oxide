use std::fs;
use std::path::Path;

use mlua::{
  Lua,
  chunk::Chunk
};

use super::{
  RuntimeOptions,
};

use super::error::{
  RuntimeError,
  InitError,
  ExecutionError,
};

#[derive(Debug)]
pub struct Runtime {
  options: RuntimeOptions,
  lua_vm:  Lua
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




